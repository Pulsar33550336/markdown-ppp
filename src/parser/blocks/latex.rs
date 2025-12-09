use nom::{
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::delimited,
    IResult, Parser,
};

use crate::ast::Block;

pub(crate) fn latex_block(input: &str) -> IResult<&str, Block> {
    map(
        delimited(tag("$$"), take_until("$$"), tag("$$")),
        |s: &str| Block::LatexBlock(s.trim().to_string()),
    )
    .parse(input)
}
