use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space0},
    combinator::{map, recognize},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

use crate::ast::Block;

pub(crate) fn macro_block(input: &str) -> IResult<&str, Block> {
    map(
        recognize(terminated(
            preceded(space0, delimited(tag("{{"), take_until("}}"), tag("}}"))),
            line_ending,
        )),
        |s: &str| {
            let content = s
                .trim()
                .trim_start_matches("{{")
                .trim_end_matches("}}")
                .trim();
            Block::MacroBlock(content.to_string())
        },
    )
    .parse(input)
}
