use crate::ast::{Block, Container};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending},
    combinator::recognize,
    multi::{many0, many_m_n, many_till},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

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
        let (input, kind) =
            line_terminated(preceded(tag(":::"), recognize(not_eof_or_eol1))).parse(input)?;
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
            blocks: blocks.into_iter().flatten().collect(),
        };

        let (input, _) = line_ending.parse(input)?;

        Ok((input, Block::Container(container)))
    }
}
