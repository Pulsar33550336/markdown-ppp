use crate::ast::*;
use crate::typst_printer::{config::*, render_typst};

#[test]
fn test_empty_document() {
    let doc = Document { blocks: vec![] };

    let result = render_typst(&doc, Config::default());
    assert_eq!(result.trim(), "");
}

#[test]
fn test_empty_paragraph() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![])],
    };

    let result = render_typst(&doc, Config::default());
    assert_eq!(result.trim(), "");
}

#[test]
fn test_empty_heading() {
    let doc = Document {
        blocks: vec![Block::Heading(Heading {
            kind: HeadingKind::Atx(1),
            content: vec![],
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("= "));
}

#[test]
fn test_empty_emphasis() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Text with ".to_string()),
            Inline::Emphasis(vec![]),
            Inline::Text(" empty emphasis.".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("__"));
}

#[test]
fn test_empty_list() {
    let doc = Document {
        blocks: vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Star),
            items: vec![],
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert_eq!(result.trim(), "");
}

#[test]
fn test_empty_list_item() {
    let doc = Document {
        blocks: vec![Block::List(List {
            kind: ListKind::Bullet(ListBulletKind::Star),
            items: vec![ListItem {
                task: None,
                blocks: vec![],
            }],
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("-"));
}

#[test]
fn test_empty_table() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![],
            alignments: vec![],
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#table"));
}

#[test]
fn test_empty_code_block() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced { info: None },
            literal: "".to_string(),
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("```\n\n```"));
}

#[test]
fn test_empty_blockquote() {
    let doc = Document {
        blocks: vec![Block::BlockQuote(vec![])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains(">"));
}

#[test]
fn test_whitespace_only_text() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("   ".to_string()),
            Inline::Text("\t\n".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("   "));
    assert!(result.contains("\t "));
}

#[test]
fn test_special_chars_in_urls() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
            destination: "https://example.com/path?q=a&b=c#fragment".to_string(),
            title: None,
            children: vec![Inline::Text("link".to_string())],
        })])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#link(\"https://example.com/path?q=a&b=c#fragment\")"));
}

#[test]
fn test_special_chars_in_code() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Code(
            "* _ \\ \"".to_string(),
        )])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("`* _ \\ \"`"));
}

#[test]
fn test_unicode_characters() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Unicode: αβγ 中文 🚀 ñáéíóú".to_string(),
        )])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("Unicode: αβγ 中文 🚀 ñáéíóú"));
}

#[test]
fn test_deeply_nested_lists() {
    fn create_nested_list(depth: usize) -> Vec<Block> {
        if depth == 0 {
            vec![Block::Paragraph(vec![Inline::Text(
                "Deep item".to_string(),
            )])]
        } else {
            vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![ListItem {
                    task: None,
                    blocks: create_nested_list(depth - 1),
                }],
            })]
        }
    }

    let doc = Document {
        blocks: create_nested_list(5),
    };

    let result = render_typst(&doc, Config::default());
    let dash_count = result.matches("-").count();
    assert_eq!(dash_count, 5);
}

#[test]
fn test_table_with_merged_cells() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    TableCell {
                        content: vec![Inline::Text("A1".to_string())],
                        colspan: Some(2),
                        rowspan: None,
                        removed_by_extended_table: false,
                    },
                    TableCell {
                        content: vec![],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: true,
                    },
                    TableCell {
                        content: vec![Inline::Text("A3".to_string())],
                        colspan: None,
                        rowspan: Some(2),
                        removed_by_extended_table: false,
                    },
                ],
                vec![
                    TableCell {
                        content: vec![Inline::Text("B1".to_string())],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: false,
                    },
                    TableCell {
                        content: vec![Inline::Text("B2".to_string())],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: false,
                    },
                    TableCell {
                        content: vec![],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: true,
                    },
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Center, Alignment::Right],
        })],
    };

    let result = render_typst(&doc, Config::default());
    println!("{}", result);
    assert!(result.contains("table.cell(colspan: 2)[\"A1\"],"));
    assert!(result.contains("table.cell(rowspan: 2)[\"A3\"],"));
    assert!(!result.contains("[]"));
}
