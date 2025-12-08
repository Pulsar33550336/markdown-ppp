use crate::ast::*;
use crate::typst_printer::{config::*, render_typst};

#[test]
fn test_thematic_break() {
    let doc = Document {
        blocks: vec![Block::ThematicBreak],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("---"));
}

#[test]
fn test_html_block() {
    let doc = Document {
        blocks: vec![Block::HtmlBlock("<div>Raw HTML</div>".to_string())],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#raw[<div>Raw HTML</div>]"));
}

#[test]
fn test_definition_block() {
    let doc = Document {
        blocks: vec![Block::Definition(LinkDefinition {
            label: vec![Inline::Text("example".to_string())],
            destination: "https://example.com".to_string(),
            title: Some("Example Site".to_string()),
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert_eq!(result.trim(), "");
}

#[test]
fn test_footnote_definition() {
    let doc = Document {
        blocks: vec![Block::FootnoteDefinition(FootnoteDefinition {
            label: "note1".to_string(),
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "This is a footnote.".to_string(),
            )])],
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert_eq!(result.trim(), "");
}

#[test]
fn test_github_alerts() {
    let alert_types = vec![
        (GitHubAlertType::Note, "Note"),
        (GitHubAlertType::Tip, "Tip"),
        (GitHubAlertType::Important, "Important"),
        (GitHubAlertType::Warning, "Warning"),
        (GitHubAlertType::Caution, "Caution"),
    ];

    for (alert_type, expected_text) in alert_types {
        let doc = Document {
            blocks: vec![Block::GitHubAlert(GitHubAlert {
                alert_type,
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Alert content".to_string(),
                )])],
            })],
        };

        let result = render_typst(&doc, Config::default());
        assert!(result.contains("#rect"));
        assert!(result.contains(expected_text));
        assert!(result.contains("Alert\ncontent"));
    }
}

#[test]
fn test_empty_block() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![Inline::Text("Before".to_string())]),
            Block::Empty,
            Block::Paragraph(vec![Inline::Text("After".to_string())]),
        ],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("Before"));
    assert!(result.contains("After"));
    let lines: Vec<&str> = result
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert_eq!(lines.len(), 2);
}

#[test]
fn test_line_break() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Line 1".to_string()),
            Inline::LineBreak,
            Inline::Text("Line 2".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("Line 1\nLine 2"));
}

#[test]
fn test_inline_code() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Use ".to_string()),
            Inline::Code("println!()".to_string()),
            Inline::Text(" function.".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("`println!()`"));
}

#[test]
fn test_inline_html() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Some ".to_string()),
            Inline::Html("<em>HTML</em>".to_string()),
            Inline::Text(" content.".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#raw[<em>HTML</em>]"));
}

#[test]
fn test_link_reference() {
    let doc = Document {
        blocks: vec![
            Block::Definition(LinkDefinition {
                label: vec![Inline::Text("example".to_string())],
                destination: "https://example.com".to_string(),
                title: None,
            }),
            Block::Paragraph(vec![
                Inline::Text("Visit ".to_string()),
                Inline::LinkReference(LinkReference {
                    label: vec![Inline::Text("example".to_string())],
                    text: vec![Inline::Text("this site".to_string())],
                }),
                Inline::Text(".".to_string()),
            ]),
        ],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#link(\"https://example.com\")[this site]"));
}

#[test]
fn test_link_reference_unresolved() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::LinkReference(
            LinkReference {
                label: vec![Inline::Text("missing".to_string())],
                text: vec![Inline::Text("broken link".to_string())],
            },
        )])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("[broken link][missing]"));
}

#[test]
fn test_image() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
            destination: "image.png".to_string(),
            title: Some("My Image".to_string()),
            alt: "Alt text".to_string(),
        })])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#image(\"image.png\", alt: \"Alt text\")"));
}

#[test]
fn test_strikethrough() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("This is ".to_string()),
            Inline::Strikethrough(vec![Inline::Text("crossed out".to_string())]),
            Inline::Text(" text.".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#strike[crossed out]"));
}

#[test]
fn test_autolink() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Visit ".to_string()),
            Inline::Autolink("https://example.com".to_string()),
            Inline::Text(".".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#link(\"https://example.com\")"));
}

#[test]
fn test_footnote_reference() {
    let doc = Document {
        blocks: vec![
            Block::FootnoteDefinition(FootnoteDefinition {
                label: "note1".to_string(),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "Footnote content".to_string(),
                )])],
            }),
            Block::Paragraph(vec![
                Inline::Text("Text with footnote".to_string()),
                Inline::FootnoteReference("note1".to_string()),
                Inline::Text(".".to_string()),
            ]),
        ],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#footnote[Footnote content]"));
}

#[test]
fn test_footnote_reference_unresolved() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Text".to_string()),
            Inline::FootnoteReference("missing".to_string()),
            Inline::Text(".".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("[^missing]"));
}

#[test]
fn test_nested_elements() {
    let doc = Document {
        blocks: vec![
            Block::BlockQuote(vec![
                Block::Paragraph(vec![Inline::Text("Quote paragraph".to_string())]),
                Block::List(List {
                    kind: ListKind::Bullet(ListBulletKind::Dash),
                    items: vec![ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![
                            Inline::Text("Item with ".to_string()),
                            Inline::Strong(vec![Inline::Text("bold".to_string())]),
                            Inline::Text(" text".to_string()),
                        ])],
                    }],
                }),
            ]),
            Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 5 }),
                items: vec![ListItem {
                    task: Some(TaskState::Incomplete),
                    blocks: vec![
                        Block::Paragraph(vec![Inline::Text("Multi-block item".to_string())]),
                        Block::CodeBlock(CodeBlock {
                            kind: CodeBlockKind::Fenced {
                                info: Some("bash".to_string()),
                            },
                            literal: "echo 'nested code'".to_string(),
                        }),
                    ],
                }],
            }),
        ],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("> Quote paragraph"));
    assert!(result.contains("[Item with *bold* text]"));
    assert!(result.contains("[#sym.checkbox]"));
    assert!(result.contains("```bash"));
    assert!(result.contains("echo 'nested code'"));
}
