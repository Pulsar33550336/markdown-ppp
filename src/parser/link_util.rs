use nom::character::complete::{anychar, char, none_of, one_of, satisfy};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, not, peek, recognize, value, verify},
    multi::{fold_many0, many0, many1},
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::rc::Rc;

use super::MarkdownParserState;

pub(crate) fn link_label<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<crate::ast::Inline>> {
    move |input: &'a str| {
        delimited(tag("["), link_label_inner(state.clone()), tag("]")).parse(input)
    }
}

fn link_label_inner<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<crate::ast::Inline>> {
    move |input: &'a str| {
        // Parse content with balanced brackets (handles nested [...] properly)
        let (input, label) = verify(balanced_brackets_content, |s: &String| {
            s.chars().any(|c| c != ' ' && c != '\n') && s.len() < 1000
        })
        .parse(input)?;

        // Recursively parse the label content as inline elements
        let (_, label) = crate::parser::inline::inline_many1(state.clone())
            .parse(label.as_str())
            .map_err(|err| err.map_input(|_| input))?;

        Ok((input, label))
    }
}

pub(crate) fn link_title(input: &str) -> IResult<&str, String> {
    alt((
        link_title_double_quoted,
        link_title_single_quoted,
        link_title_parenthesized,
    ))
    .parse(input)
}

fn link_title_parenthesized(input: &str) -> IResult<&str, String> {
    delimited(char('('), link_title_inner(')'), char(')')).parse(input)
}

fn link_title_single_quoted(input: &str) -> IResult<&str, String> {
    delimited(char('\''), link_title_inner('\''), char('\'')).parse(input)
}

fn link_title_double_quoted(input: &str) -> IResult<&str, String> {
    delimited(tag("\""), link_title_inner('"'), tag("\"")).parse(input)
}

fn link_title_inner(end_delim: char) -> impl FnMut(&str) -> IResult<&str, String> {
    move |input: &str| {
        fold_many0(
            alt((
                map(escaped_char, |c| c.to_string()),
                map(none_of(&[end_delim, '\\'][..]), |c| c.to_string()),
            )),
            String::new,
            |mut acc, s| {
                acc.push_str(&s);
                acc
            },
        )
        .parse(input)
    }
}

fn escaped_char(input: &str) -> IResult<&str, char> {
    preceded(tag("\\"), anychar).parse(input)
}

/// Maximum nesting depth for square brackets to prevent stack overflow.
const MAX_BRACKET_DEPTH: usize = 32;

/// Parses content inside square brackets, handling nested brackets and escapes.
/// Returns the raw string content (including nested bracket pairs).
/// Escaped brackets (\[ and \]) are converted to their literal characters.
fn balanced_brackets_content(input: &str) -> IResult<&str, String> {
    balanced_brackets_content_with_depth(input, 0)
}

/// Internal implementation with depth tracking to prevent stack overflow.
fn balanced_brackets_content_with_depth(input: &str, depth: usize) -> IResult<&str, String> {
    fold_many0(
        move |i| {
            alt((
                // Escaped ] - needed for balanced bracket parsing (consume backslash)
                map(preceded(char('\\'), char(']')), |c| c.to_string()),
                // Other escaped characters (including \[) - preserve backslash for inline parsing
                map(escaped_char, |c| format!("\\{c}")),
                // Nested brackets - recursively parse if depth allows
                move |i| {
                    if depth < MAX_BRACKET_DEPTH {
                        balanced_brackets_with_depth(i, depth).map(|(i, s)| (i, format!("[{s}]")))
                    } else {
                        // At max depth, treat [ as a literal character
                        map(char('['), |c| c.to_string()).parse(i)
                    }
                },
                // Any character except [ ] \
                map(satisfy(|c| c != '[' && c != ']' && c != '\\'), |c| {
                    c.to_string()
                }),
            ))
            .parse(i)
        },
        String::new,
        |mut acc, item| {
            acc.push_str(&item);
            acc
        },
    )
    .parse(input)
}

/// Parses a balanced pair of square brackets: [content]
/// Returns the content without the outer brackets.
fn balanced_brackets_with_depth(input: &str, depth: usize) -> IResult<&str, String> {
    let (input, _) = char('[').parse(input)?;
    let (input, content) = balanced_brackets_content_with_depth(input, depth + 1)?;
    let (input, _) = char(']').parse(input)?;
    Ok((input, content))
}

pub(crate) fn link_destination(input: &str) -> IResult<&str, String> {
    alt((link_destination1, link_destination2)).parse(input)
}

fn link_destination1(input: &str) -> IResult<&str, String> {
    let (input, _) = char('<').parse(input)?;

    let (input, chars) = many0(alt((
        preceded(char('\\'), one_of("<>")),
        preceded(peek(not(one_of("\n<>"))), anychar),
    )))
    .parse(input)?;
    let (input, _) = char('>').parse(input)?;

    let v: String = chars.iter().collect();

    Ok((input, v))
}

fn link_destination2(input: &str) -> IResult<&str, String> {
    let (input, _) = peek(satisfy(|c| is_valid_char(c) && c != '<')).parse(input)?;

    map(
        recognize(many1(alt((
            value((), escaped_char),
            value((), balanced_parens),
            value((), satisfy(|c| is_valid_char(c) && c != '(' && c != ')')),
        )))),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

fn balanced_parens(input: &str) -> IResult<&str, String> {
    delimited(
        tag("("),
        map(
            fold_many0(
                alt((
                    map(escaped_char, |c| c.to_string()),
                    map(balanced_parens, |s| format!("({s})")),
                    map(satisfy(|c| is_valid_char(c) && c != '(' && c != ')'), |c| {
                        c.to_string()
                    }),
                )),
                String::new,
                |mut acc, item| {
                    acc.push_str(&item);
                    acc
                },
            ),
            |s| s,
        ),
        tag(")"),
    )
    .parse(input)
}

fn is_valid_char(c: char) -> bool {
    !c.is_ascii_control() && c != ' ' && c != '<'
}
