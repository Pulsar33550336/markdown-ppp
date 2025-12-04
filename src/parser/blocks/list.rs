use crate::ast::{ListBulletKind, ListItem, ListKind, ListOrderedKindOptions, TaskState};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{char, line_ending, one_of},
    combinator::{map, opt, value},
    multi::{many0, many_m_n},
    sequence::{delimited, terminated},
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

fn parse_list_marker(input: &str) -> IResult<&str, (ListKind, usize, Option<TaskState>, &str)> {
    let (input, indent) = many_m_n(0, 3, char(' ')).parse(input)?;
    let (marker_end, kind) = list_marker(input)?;
    let marker_len = input.len() - marker_end.len();

    let (input, (whitespace, task, content)) =
        alt((parse_list_marker_rest_with_content, parse_list_marker_empty))
            .parse(marker_end)?;

    let prefix_len = indent.len() + marker_len + whitespace;
    Ok((input, (kind, prefix_len, task, content)))
}

fn parse_list_marker_rest_with_content(
    input: &str,
) -> IResult<&str, (usize, Option<TaskState>, &str)> {
    let (input, whitespace) = many_m_n(1, 4, char(' ')).parse(input)?;
    let (input, task) = opt(terminated(list_item_task_state, char(' '))).parse(input)?;
    let (input, content) = not_eof_or_eol0(input)?;
    Ok((input, (whitespace.len(), task, content)))
}

fn parse_list_marker_empty(input: &str) -> IResult<&str, (usize, Option<TaskState>, &str)> {
    let (input, _) = eof_or_eol(input)?;
    Ok((input, (1, None, "")))
}

pub(crate) fn list(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, crate::ast::List> {
    move |input: &str| {
        let mut items = Vec::new();
        let mut current_item_content = String::new();

        let (mut i, (kind, mut prefix_len, task, first_line)) =
            parse_list_marker(input)?;
        items.push(ListItem {
            task,
            blocks: Vec::new(),
        });
        current_item_content.push_str(first_line);

        let mut blank_line_seen = first_line.trim().is_empty();

        loop {
            if i.is_empty() {
                break;
            }

            if let Ok((next_i, (next_kind, p_len, task, first_line))) =
                parse_list_marker(i)
            {
                if std::mem::discriminant(&kind) == std::mem::discriminant(&next_kind) {
                    let (_, blocks) =
                        many0(crate::parser::blocks::block(state.clone()))
                            .parse(&current_item_content)
                            .map_err(|err| err.map_input(|_| i))?;
                    items.last_mut().unwrap().blocks =
                        blocks.into_iter().flatten().collect();
                    current_item_content.clear();

                    prefix_len = p_len;
                    items.push(ListItem {
                        task,
                        blocks: Vec::new(),
                    });
                    current_item_content.push_str(first_line);
                    blank_line_seen = first_line.trim().is_empty();
                    i = next_i;
                    continue;
                }
            }

            let (after_line, line) = not_eof_or_eol0(i)?;
            let (next_i, _) = opt(line_ending).parse(after_line)?;

            if line.trim().is_empty() {
                blank_line_seen = true;
                current_item_content.push('\n');
                i = next_i;
                continue;
            }

            let indent_len = line.chars().take_while(|c| *c == ' ').count();
            if indent_len >= prefix_len {
                let to_add = &line[prefix_len..];
                if !current_item_content.is_empty() {
                    current_item_content.push('\n');
                }
                current_item_content.push_str(to_add);
                blank_line_seen = false;
                i = next_i;
                continue;
            }

            if blank_line_seen {
                if crate::parser::blocks::thematic_break::thematic_break(state.clone())(line).is_ok() {
                    break;
                }
                if !current_item_content.is_empty() {
                    current_item_content.push('\n');
                }
                current_item_content.push_str(line);
                blank_line_seen = false;
                i = next_i;
                continue;
            }

            // This is the crucial part for simple continuation.
            // If it's not a blank line, not indented, and not a new list item,
            // but the previous line was NOT blank, it's part of the same paragraph.
            if !blank_line_seen {
                 if let Ok((_, (next_kind, _, _, _))) = parse_list_marker(i) {
                     if std::mem::discriminant(&kind) != std::mem::discriminant(&next_kind) {
                         // This is a new list of a different kind. Break and let it be parsed as a nested block.
                     } else {
                        // This case should be handled by the first `if let` in the loop.
                        // Breaking here is safe.
                        break;
                     }
                 }

                // It is not a new list item. It's a continuation of the current paragraph.
                if !current_item_content.is_empty() {
                    current_item_content.push('\n');
                }
                current_item_content.push_str(line);
                i = next_i;
                continue;
            }


            break;
        }

        if !current_item_content.is_empty() {
            let (_, blocks) =
                many0(crate::parser::blocks::block(state.clone()))
                    .parse(&current_item_content)
                    .map_err(|err| err.map_input(|_| i))?;
            items.last_mut().unwrap().blocks = blocks.into_iter().flatten().collect();
        }

        let list = crate::ast::List { kind, items };
        Ok((i, list))
    }
}
