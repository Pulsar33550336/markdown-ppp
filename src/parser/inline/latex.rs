use nom::{
    bytes::complete::take_while,
    character::complete::char,
    combinator::map,
    sequence::delimited,
    IResult, Parser,
};

use crate::ast::Inline;

pub(crate) fn latex(input: &str) -> IResult<&str, Vec<Inline>> {
    map(
        delimited(
            char('$'),
            take_while(|c| c != '$'),
            char('$'),
        ),
        |s: &str| vec![Inline::Latex(s.to_string())],
    )
    .parse(input)
}
