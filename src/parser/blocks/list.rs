use crate::ast::{ListBulletKind, ListItem, ListKind, ListOrderedKindOptions, TaskState};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::combinator::verify;
use nom::{
    branch::alt,
    character::complete::{char, one_of, space0},
    combinator::{map, opt, peek, recognize, value},
    multi::{many0, many_m_n},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use std::rc::Rc;

fn list_item_task_state(input: &str) -> IResult<&str, TaskState> {
    delimited(
        char('['),
        alt((
            value(TaskState::Complete, one_of("xX")),
            value(TaskState::Incomplete, char(' ')),
        )),
        char(']'),
    )
    .parse(input)
}

fn list_marker(input: &str) -> IResult<&str, ListKind> {
    alt((
        list_marker_ordered,
        list_marker_star,
        list_marker_plus,
        list_marker_dash,
    ))
    .parse(input)
}

fn list_marker_star(input: &str) -> IResult<&str, ListKind> {
    map(char('*'), |_| ListKind::Bullet(ListBulletKind::Star)).parse(input)
}

fn list_marker_plus(input: &str) -> IResult<&str, ListKind> {
    map(char('+'), |_| ListKind::Bullet(ListBulletKind::Plus)).parse(input)
}

fn list_marker_dash(input: &str) -> IResult<&str, ListKind> {
    map(char('-'), |_| ListKind::Bullet(ListBulletKind::Dash)).parse(input)
}

fn list_marker_ordered(input: &str) -> IResult<&str, ListKind> {
    map(
        terminated(nom::character::complete::u64, one_of(".)")),
        |start| ListKind::Ordered(ListOrderedKindOptions { start }),
    )
    .parse(input)
}

fn list_marker_followed_by_spaces(
    input: &str,
) -> IResult<&str, (ListKind, usize, Option<TaskState>)> {
    let (remaining, kind) = delimited(
        many_m_n(0, 3, char(' ')),
        list_marker,
        many_m_n(1, 4, char(' ')),
    )
    .parse(input)?;

    let consumed = input.len() - remaining.len();

    let (input, task_state) = opt(terminated(list_item_task_state, char(' '))).parse(remaining)?;

    Ok((input, (kind, consumed, task_state)))
}

fn list_marker_followed_by_newline(
    input: &str,
) -> IResult<&str, (ListKind, usize, Option<TaskState>)> {
    let (remaining, kind) = preceded(many_m_n(0, 3, char(' ')), list_marker).parse(input)?;

    // Cases:
    // 1.
    // 1.____
    if let Ok((tail, _)) = line_terminated(space0).parse(remaining) {
        // Calculate prefix length: consumed + 1 space
        let consumed = input.len() - remaining.len() + 1;

        return Ok((tail, (kind, consumed, None)));
    }

    let (remaining, _) = many_m_n(0, 3, char(' ')).parse(remaining)?;
    let consumed = input.len() - remaining.len() + 1;

    let (remaining, task_state) = line_terminated(list_item_task_state).parse(remaining)?;

    Ok((remaining, (kind, consumed, Some(task_state))))
}

pub(crate) fn list_marker_with_span_size(
    input: &str,
) -> IResult<&str, (ListKind, usize, Option<TaskState>, String)> {
    alt((
        map(
            list_marker_followed_by_newline,
            |(list_kind, prefix_length, task_state)| {
                (list_kind, prefix_length, task_state, String::new())
            },
        ),
        (map(
            (
                list_marker_followed_by_spaces,
                line_terminated(not_eof_or_eol0),
            ),
            |((list_kind, prefix_length, task_state), s)| {
                (list_kind, prefix_length, task_state, s.to_string())
            },
        )),
    ))
    .parse(input)
}

fn list_item_lines(
    state: Rc<MarkdownParserState>,
    list_kind: ListKind,
    prefix_length: usize,
) -> impl FnMut(&str) -> IResult<&str, Vec<Vec<&str>>> {
    move |mut input: &str| {
        let mut lines = Vec::new();
        let mut was_blank = false;

        loop {
            if input.is_empty() {
                break;
            }

            let marker_parser = match list_kind {
                ListKind::Ordered(_) => list_marker_ordered,
                ListKind::Bullet(ListBulletKind::Star) => list_marker_star,
                ListKind::Bullet(ListBulletKind::Plus) => list_marker_plus,
                ListKind::Bullet(ListBulletKind::Dash) => list_marker_dash,
            };

            // Terminator check
            if peek(alt((
                value(
                    (),
                    crate::parser::blocks::thematic_break::thematic_break(state.clone()),
                ),
                value(
                    (),
                    (
                        verify(
                            recognize(many_m_n(0, prefix_length, char(' '))),
                            |indent: &str| indent.len() < prefix_length,
                        ),
                        marker_parser,
                    ),
                ),
            )))
            .parse(input)
            .is_ok()
            {
                break;
            }

            // Indented line
            let mut indented_parser = preceded(
                many_m_n(prefix_length, prefix_length, char(' ')),
                line_terminated(not_eof_or_eol0),
            );
            if let Ok((rem, content)) = indented_parser.parse(input) {
                lines.push(vec![content]);
                input = rem;
                was_blank = false;
                continue;
            }

            // Blank line
            let mut blank_line_parser = recognize(line_terminated(space0));
            if let Ok((rem, content)) = blank_line_parser.parse(input) {
                if content.trim().is_empty() {
                    lines.push(vec![content]);
                    input = rem;
                    was_blank = true;
                    continue;
                }
            }

            // Lazy continuation
            if was_blank {
                // After a blank line, lazy continuation is only allowed for block-level elements,
                // not paragraphs. We'll approximate this by checking for a list marker.
                if peek(list_marker).parse(input).is_ok() {
                    if let Ok((rem, content)) = line_terminated(not_eof_or_eol0).parse(input) {
                        lines.push(vec![content]);
                        input = rem;
                        was_blank = false;
                        continue;
                    }
                }
            } else {
                // Continuation of a paragraph. This is a non-indented line that is not preceded
                // by a blank line.
                if let Ok((rem, content)) = line_terminated(not_eof_or_eol0).parse(input) {
                    lines.push(vec![content]);
                    input = rem;
                    // was_blank remains false
                    continue;
                }
            }

            // If nothing matches, we're done with this list item
            break;
        }

        Ok((input, lines))
    }
}
pub(crate) fn list_item(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, (ListKind, ListItem)> {
    move |input: &str| {
        let (input, (list_kind, item_prefix_length, task_state, first_line)) =
            list_marker_with_span_size(input)?;

        let (input, rest_lines) =
            list_item_lines(state.clone(), list_kind.clone(), item_prefix_length).parse(input)?;

        let total_size = first_line.len() + rest_lines.len();
        let mut item_content = String::with_capacity(total_size);
        if !first_line.is_empty() {
            item_content.push_str(&first_line)
        }
        for line in rest_lines {
            item_content.push('\n');
            for subline in line {
                item_content.push_str(subline)
            }
        }

        let (_, blocks) = many0(crate::parser::blocks::block(state.clone()))
            .parse(&item_content)
            .map_err(|err| err.map_input(|_| input))?;

        let blocks = blocks.into_iter().flatten().collect();

        let item = ListItem {
            task: task_state,
            blocks,
        };
        Ok((input, (list_kind, item)))
    }
}

pub(crate) fn list(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, crate::ast::List> {
    move |input: &str| {
        let (mut i, (first_kind, first_item)) = list_item(state.clone())(input)?;
        let mut items = vec![first_item];

        loop {
            if i.is_empty() {
                break;
            }

            match list_item(state.clone())(i) {
                Ok((rem, (kind, item))) => {
                    // CommonMark spec says lists can't have items of different types.
                    // If the type is different, we end this list.
                    if std::mem::discriminant(&kind) == std::mem::discriminant(&first_kind) {
                        items.push(item);
                        i = rem;
                    } else {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        let list = crate::ast::List {
            kind: first_kind,
            items,
        };

        Ok((i, list))
    }
}
