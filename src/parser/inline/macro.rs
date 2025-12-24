use nom::{
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::delimited,
    IResult, Parser,
};

use crate::ast::Inline;

pub(crate) fn macro_parser(input: &str) -> IResult<&str, Vec<Inline>> {
    map(
        delimited(tag("{{"), take_until("}}"), tag("}}")),
        |s: &str| vec![Inline::Macro(s.trim().to_string())],
    )
    .parse(input)
}
