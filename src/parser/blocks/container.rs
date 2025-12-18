use crate::ast::{Block, Container};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{
        anychar, char, line_ending, multispace0, multispace1, not_line_ending, space0,
    },
    combinator::{cut, map, recognize},
    multi::{many0, many_m_n, many_till, separated_list0},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult, Parser,
};
use std::rc::Rc;

fn parse_quoted_string<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    delimited(char('"'), is_not("\""), char('"')).parse(input)
}

fn parse_unquoted_string<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_').parse(input)
}

fn parse_value<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    alt((parse_quoted_string, parse_unquoted_string)).parse(input)
}

fn parse_key_value_pair<'a>(input: &'a str) -> IResult<&'a str, (String, String)> {
    map(
        separated_pair(
            take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_'),
            tuple((space0, char('='), space0)),
            cut(parse_value),
        ),
        |(k, v): (&str, &str)| (k.to_owned(), v.to_owned()),
    )
    .parse(input)
}

fn parse_container_params<'a>(input: &'a str) -> IResult<&'a str, Vec<(String, String)>> {
    delimited(
        char('{'),
        preceded(
            multispace0,
            separated_list0(multispace1, parse_key_value_pair),
        ),
        preceded(multispace0, char('}')),
    )
    .parse(input)
}

pub(crate) fn container<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Block> {
    move |input: &'a str| {
        if !state.containers.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }

        let (input, _) = many_m_n(0, 3, char(' ')).parse(input)?;
        let (input, line) = line_terminated(preceded(tag(":::"), not_line_ending)).parse(input)?;

        let (remainder, kind) = recognize(is_not("{ \t\r\n")).parse(line)?;
        let (remainder, _) = space0(remainder)?;

        let (remainder, params) = if remainder.starts_with('{') {
            parse_container_params(remainder)?
        } else {
            (remainder, vec![])
        };

        if !remainder.trim().is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }

        let kind_trimmed = kind.trim();

        let mut nested_state = state.nested();
        nested_state.containers.push(kind_trimmed.to_string());
        let nested_state_rc = Rc::new(nested_state);

        let (input, (chars, _)) = many_till(
            anychar,
            preceded(many_m_n(0, 3, char(' ')), tag(":::")),
        )
        .parse(input)?;

        let inner_content: String = chars.into_iter().collect();
        let (_, blocks) = many0(crate::parser::blocks::block(nested_state_rc))
            .parse(&inner_content)
            .map_err(|err| err.map_input(|_| input))?;

        let container = Container {
            kind: kind_trimmed.to_owned(),
            params,
            blocks: blocks.into_iter().flatten().collect(),
        };

        let (input, _) = line_ending(input)?;

        Ok((input, Block::Container(container)))
    }
}
