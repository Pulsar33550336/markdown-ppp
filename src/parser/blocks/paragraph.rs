use crate::ast::Inline;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{char, line_ending, space0},
    combinator::{not, peek, value},
    multi::{many_m_n, separated_list0},
    sequence::preceded,
    IResult, Parser,
};
use std::borrow::Cow;
use std::rc::Rc;

pub(crate) fn paragraph<'a>(
    state: Rc<MarkdownParserState>,
    check_first_line: bool,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Inline>> {
    move |input: &'a str| {
        let mut lines = Vec::new();
        let input = if check_first_line {
            input
        } else {
            // Skip checks for the first line, just make it a paragraph
            let (input, first_line) =
                preceded(many_m_n(0, 3, char(' ')), not_eof_or_eol1).parse(input)?;
            lines.push(first_line);
            input
        };

        let paragraph_parser = separated_list0(
            line_ending,
            preceded(
                is_paragraph_line_start(state.clone()),
                preceded(many_m_n(0, 3, char(' ')), not_eof_or_eol1),
            ),
        );
        let (input, rest_lines) = line_terminated(paragraph_parser).parse(input)?;
        lines.extend(rest_lines);

        let content = lines.join("\n");

        let transformed_input =
            if let Some(inline_macro_replacer) = &state.config.inline_macro_replacer {
                let mut replacer = inline_macro_replacer.borrow_mut();
                let mut result = String::new();
                let mut last_pos = 0;

                while let Some(start_pos) = content[last_pos..].find("{{") {
                    let absolute_start = last_pos + start_pos;
                    let mut balance = 1;
                    let mut current_scan_pos = absolute_start + 2;
                    let mut end_pos = None;

                    while let Some(next_marker_pos) =
                        content[current_scan_pos..].find(|c| c == '{' || c == '}')
                    {
                        let absolute_marker_pos = current_scan_pos + next_marker_pos;
                        if content.get(absolute_marker_pos..absolute_marker_pos + 2) == Some("{{")
                        {
                            balance += 1;
                            current_scan_pos = absolute_marker_pos + 2;
                        } else if content.get(absolute_marker_pos..absolute_marker_pos + 2)
                            == Some("}}")
                        {
                            balance -= 1;
                            if balance == 0 {
                                end_pos = Some(absolute_marker_pos);
                                break;
                            }
                            current_scan_pos = absolute_marker_pos + 2;
                        } else {
                            current_scan_pos = absolute_marker_pos + 1;
                        }
                    }

                    if let Some(absolute_end) = end_pos {
                        result.push_str(&content[last_pos..absolute_start]);
                        let macro_content = &content[absolute_start + 2..absolute_end];
                        let replacement = (replacer)(macro_content.trim());
                        result.push_str(&replacement);
                        last_pos = absolute_end + 2;
                    } else {
                        // No matching end found, stop processing
                        break;
                    }
                }

                result.push_str(&content[last_pos..]);
                Cow::Owned(result)
            } else {
                Cow::Borrowed(content.as_str())
            };

        let (_, content) = crate::parser::inline::inline_many1(state.clone())
            .parse(transformed_input.as_ref())
            .map_err(|err| err.map_input(|_| input))?;

        Ok((input, content))
    }
}

pub(crate) fn is_paragraph_line_start<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    move |input: &'a str| {
        peek(not(alt((
            conditional_block_unit(
                state.config.block_heading_v1_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::heading::heading_v1(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_heading_v2_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::heading::heading_v2_level(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_thematic_break_behavior.clone(),
                crate::parser::blocks::thematic_break::thematic_break(state.clone()),
            ),
            conditional_block_unit(
                state.config.block_blockquote_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::blockquote::blockquote(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_list_behavior.clone(),
                value((), crate::parser::blocks::list::list_item(state.clone())),
            ),
            conditional_block_unit(
                state.config.block_code_block_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::code_block::code_block_fenced(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_html_block_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::html_block::html_block(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_link_definition_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::link_definition::link_definition(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_footnote_definition_behavior.clone(),
                value(
                    (),
                    crate::parser::blocks::footnote_definition::footnote_definition(state.clone()),
                ),
            ),
            conditional_block_unit(
                state.config.block_table_behavior.clone(),
                value((), crate::parser::blocks::table::table(state.clone())),
            ),
            value(
                vec![()],
                crate::parser::blocks::custom_parser(state.clone()),
            ),
            value(vec![()], line_terminated(space0)),
        ))))
        .parse(input)
    }
}
