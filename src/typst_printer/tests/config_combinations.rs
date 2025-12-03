use crate::ast::*;
use crate::typst_printer::{config::*, render_typst};

#[test]
fn test_width_configuration() {
    let long_text = "This is a very long line of text that should be wrapped at different widths depending on the configuration settings.";
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(long_text.to_string())])],
    };

    let widths = vec![40, 80, 120];
    let mut results = Vec::new();

    for width in widths {
        let config = Config::default().with_width(width);
        let result = render_typst(&doc, config);
        results.push(result);
    }

    let line_counts: Vec<usize> = results.iter().map(|r| r.lines().count()).collect();
    assert!(line_counts[0] >= line_counts[1]);
    assert!(line_counts[1] >= line_counts[2]);
}

#[test]
fn test_config_builder_pattern() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text("Test".to_string())])],
    };

    let config = Config::default().with_width(60);

    let result = render_typst(&doc, config);
    assert!(result.contains("Test"));
}

#[test]
fn test_default_config() {
    let doc = Document {
        blocks: vec![
            Block::Table(Table {
                rows: vec![vec![TableCell {
                    content: vec![Inline::Text("Test".to_string())],
                    colspan: None,
                    rowspan: None,
                    removed_by_extended_table: false,
                }]],
                alignments: vec![Alignment::Left],
            }),
            Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_string()),
                },
                literal: "test".to_string(),
            }),
        ],
    };

    let result = render_typst(&doc, Config::default());

    assert!(result.contains("#figure(table"));
    assert!(result.contains("```rust"));
}