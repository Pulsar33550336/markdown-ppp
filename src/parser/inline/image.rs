use crate::parser::link_util::link_title;
use crate::parser::MarkdownParserState;
use crate::{
    ast::{Image, ImageAttributes, Inline},
    parser::link_util::link_destination,
};
use nom::{
    bytes::complete::{take_until, take_while},
    character::complete::{alpha1, char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};
use std::rc::Rc;

fn key_value_parser<'a>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    separated_pair(
        preceded(multispace0, alpha1),
        delimited(multispace0, char('='), multispace0),
        delimited(char('"'), take_until("\""), char('"')),
    )
    .parse(input)
}

fn attributes_parser<'a>(input: &'a str) -> IResult<&'a str, ImageAttributes> {
    map(
        delimited(
            preceded(multispace0, char('{')),
            preceded(multispace0, separated_list0(multispace1, key_value_parser)),
            preceded(multispace0, char('}')),
        ),
        |key_values| {
            let mut attrs = ImageAttributes::default();
            for (key, value) in key_values {
                match key {
                    "width" => attrs.width = Some(value.to_string()),
                    "height" => attrs.height = Some(value.to_string()),
                    _ => {}
                }
            }
            attrs
        },
    )
    .parse(input)
}

// ![alt text](/url "title")
pub(crate) fn image<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, alt) = preceded(
            char('!'),
            delimited(char('['), take_while(|c| c != ']'), char(']')),
        )
        .parse(input)?;

        let (input, (destination, title)) = delimited(
            char('('),
            (
                preceded(multispace0, link_destination),
                opt(preceded(multispace0, link_title)),
            ),
            preceded(multispace0, char(')')),
        )
        .parse(input)?;

        let (input, attr) = opt(attributes_parser).parse(input)?;

        Ok((
            input,
            Inline::Image(Image {
                destination,
                title,
                alt: alt.to_owned(),
                attr,
            }),
        ))
    }
}
