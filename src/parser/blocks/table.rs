use super::{eof_or_eol, line_terminated};
use crate::ast::{Alignment, Inline, Table, TableCell, TableRow};
use crate::parser::MarkdownParserState;
use nom::multi::many_m_n;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, space0},
    combinator::{map, not, opt, recognize, value, peek},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn table<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Table> {
    move |input: &'a str| {
        let (input, header) = parse_table_row(state.clone()).parse(input)?;
        let col_count = header.len();

        let (input, alignments) = parse_alignment_row.parse(input)?;
        if alignments.len() != col_count {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }

        let (input, rows) = parse_table_data_rows(state.clone(), col_count).parse(input)?;

        let mut all_rows = std::iter::once(header).chain(rows).collect::<Vec<_>>();
        process_spans(&mut all_rows);

        Ok((
            input,
            Table {
                rows: all_rows,
                alignments,
            },
        ))
    }
}

fn process_spans(rows: &mut Vec<TableRow>) {
    // Process rowspans first, column by column
    if rows.len() > 1 && !rows.is_empty() && !rows[0].is_empty() {
        for i in 0..rows[0].len() {
            // for each column
            for j in 1..rows.len() {
                // for each row, starting from the second
                if let Some(cell) = rows[j].get(i) {
                    if !cell.removed_by_extended_table
                        && cell.content == vec![Inline::Text("^".to_string())]
                    {
                        // Find the cell above to merge into
                        let mut target_row_idx = j - 1;
                        loop {
                            if let Some(target_cell) = rows.get(target_row_idx).and_then(|r| r.get(i))
                            {
                                if !target_cell.removed_by_extended_table {
                                    // Found it.
                                    let source_rowspan = rows[j][i].rowspan.unwrap_or(1);
                                    let target_cell_mut = &mut rows[target_row_idx][i];
                                    let target_rowspan = target_cell_mut.rowspan.get_or_insert(1);
                                    *target_rowspan += source_rowspan;

                                    rows[j][i].removed_by_extended_table = true;
                                    break;
                                }
                            }
                            if target_row_idx == 0 {
                                break;
                            }
                            target_row_idx -= 1;
                        }
                    }
                }
            }
        }
    }

    // Now process colspans, row by row
    for row in rows.iter_mut() {
        if !row.is_empty() {
            for i in 1..row.len() {
                // for each cell, starting from second
                if let Some(cell) = row.get(i) {
                    if !cell.removed_by_extended_table
                        && cell.content == vec![Inline::Text("<".to_string())]
                    {
                        // find cell to the left to merge into
                        let mut target_col_idx = i - 1;
                        loop {
                            if let Some(target_cell) = row.get(target_col_idx) {
                                if !target_cell.removed_by_extended_table {
                                    // Found it.
                                    let source_colspan = row[i].colspan.unwrap_or(1);
                                    let target_cell_mut = &mut row[target_col_idx];
                                    let target_colspan = target_cell_mut.colspan.get_or_insert(1);
                                    *target_colspan += source_colspan;

                                    row[i].removed_by_extended_table = true;
                                    break;
                                }
                            }
                            if target_col_idx == 0 {
                                break;
                            }
                            target_col_idx -= 1;
                        }
                    }
                }
            }
        }
    }
}

fn parse_table_data_rows<'a>(
    state: Rc<MarkdownParserState>,
    col_count: usize,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<TableRow>> {
    move |input: &'a str| {
        many0(map(parse_table_row(state.clone()), move |mut row| {
            match row.len().cmp(&col_count) {
                std::cmp::Ordering::Less => {
                    row.extend((0..(col_count - row.len())).map(|_| TableCell {
                        content: vec![Inline::Text(String::new())],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: false,
                    }));
                }
                std::cmp::Ordering::Greater => {
                    row.truncate(col_count);
                }
                _ => {}
            }
            row
        }))
        .parse(input)
    }
}

fn parse_alignment_row(input: &str) -> IResult<&str, Vec<Alignment>> {
    fn parse_cell_alignment(cell: &str) -> Alignment {
        let trimmed = cell.trim();
        let starts_with_colon = trimmed.starts_with(':');
        let ends_with_colon = trimmed.ends_with(':');

        match (starts_with_colon, ends_with_colon) {
            (true, true) => Alignment::Center,
            (true, false) => Alignment::Left,
            (false, true) => Alignment::Right,
            (false, false) => Alignment::None,
        }
    }

    let alignment_parser = delimited(
        space0,
        alt((
            recognize(delimited(char(':'), many1(char('-')), char(':'))),
            recognize(preceded(char(':'), many1(char('-')))),
            recognize(terminated(many1(char('-')), char(':'))),
            recognize(many1(char('-'))),
        )),
        space0,
    );

    line_terminated(preceded(
        many_m_n(0, 3, char(' ')),
        delimited(
            char('|'),
            separated_list1(char('|'), map(alignment_parser, parse_cell_alignment)),
            opt(char('|')),
        ),
    ))
    .parse(input)
}

fn parse_table_row<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, TableRow> {
    move |input: &'a str| {
        line_terminated(preceded(
            many_m_n(0, 3, char(' ')),
            delimited(
                char('|'),
                separated_list1(char('|'), cell_content(state.clone())),
                opt(char('|')),
            ),
        ))
        .parse(input)
    }
}

fn cell_content<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, TableCell> {
    move |input: &'a str| {
        let (input, chars) = many1(preceded(
            not(alt((value((), eof_or_eol), value((), char('|'))))),
            alt((value('|', tag("\\|")), anychar)),
        ))
        .parse(input)?;

        let content = chars.iter().collect::<String>();
        let trimmed_content = content.trim();
        let (_, content) = crate::parser::inline::inline_many0(state.clone())
            .parse(trimmed_content)
            .map_err(|err| err.map_input(|_| input))?;

        Ok((
            input,
            TableCell {
                content,
                colspan: None,
                rowspan: None,
                removed_by_extended_table: false,
            },
        ))
    }
}
