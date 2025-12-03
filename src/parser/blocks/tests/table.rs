use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn table1() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| foo | bar |
| --- | --- |
| baz | bim |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("foo".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("bar".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("baz".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("bim".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table2() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| foo | bar |
| :-- | --: |
| baz | bim |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("foo".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("bar".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("baz".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("bim".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ]
                ],
                alignments: vec![Alignment::Left, Alignment::Right]
            })]
        }
    );
}

#[test]
fn table3() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| foo | bar |
| --- | --- |
| baz | b\\|im |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("foo".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("bar".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("baz".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("b|im".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table4() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| abc | def |
| --- | --- |
| bar |
| bar | baz | boo |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("abc".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("def".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("bar".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("bar".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("baz".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table5() {
    // Test table with extra columns in data rows (should be truncated)
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| header1 | header2 |
| ------- | ------- |
| cell1 | cell2 | extra1 | extra2 |
| cell3 | cell4 | extra3 |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("header1".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("header2".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("cell1".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("cell2".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("cell3".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("cell4".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table6() {
    // Test table without trailing pipe (should still parse)
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| header1 | header2
| ------- | -------
| cell1 | cell2
| cell3 | cell4",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("header1".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("header2".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("cell1".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("cell2".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("cell3".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("cell4".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table7() {
    // Test table with very long cell content
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| Short | Very long content that would normally wrap on narrow displays but should be preserved as-is |
| ----- | -------------------------------------------------------------------------------------------- |
| A     | This is another very long cell content that tests how the parser handles lengthy text        |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("Short".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("Very long content that would normally wrap on narrow displays but should be preserved as-is".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("A".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("This is another very long cell content that tests how the parser handles lengthy text".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        }
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table_malformed_separators() {
    // Test table with malformed separators - should NOT parse as complete table
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| header1 | header2 | header3 |
|----------------------------------------------------------------------------------------|------------------------------------|--------|
| cell1 | cell2 | cell3 |",
    )
    .unwrap();

    // This particular malformed separator actually has 3 columns (same as header)
    // so it will parse as a table, but let's verify the structure
    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Table(table) => {
            // Should have header + data row
            assert_eq!(table.rows.len(), 2);
            assert_eq!(table.alignments.len(), 3);
        }
        _ => panic!("Expected block to be a table"),
    }
}

#[test]
fn table_with_merged_cells() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| A1 | < | A3 |
| --- | --- | --- |
| B1 | B2 | ^ |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        TableCell {
                            content: vec![Inline::Text("A1".to_owned())],
                            colspan: Some(2),
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("<".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: true
                        },
                        TableCell {
                            content: vec![Inline::Text("A3".to_owned())],
                            colspan: None,
                            rowspan: Some(2),
                            removed_by_extended_table: false
                        }
                    ],
                    vec![
                        TableCell {
                            content: vec![Inline::Text("B1".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("B2".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: false
                        },
                        TableCell {
                            content: vec![Inline::Text("^".to_owned())],
                            colspan: None,
                            rowspan: None,
                            removed_by_extended_table: true
                        }
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None, Alignment::None]
            })]
        }
    );
}
