use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn image_with_attributes() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        r#"![foo](/url){width="100pt" height="50pt"}"#,
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "/url".to_owned(),
                title: None,
                alt: "foo".to_owned(),
                attr: Some(ImageAttributes {
                    width: Some("100pt".to_owned()),
                    height: Some("50pt".to_owned()),
                }),
            })])]
        }
    );
}

#[test]
fn image_with_attributes_and_title() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        r#"![foo](/url "title"){width="100pt" height="50pt"}"#,
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "/url".to_owned(),
                title: Some("title".to_owned()),
                alt: "foo".to_owned(),
                attr: Some(ImageAttributes {
                    width: Some("100pt".to_owned()),
                    height: Some("50pt".to_owned()),
                }),
            })])]
        }
    );
}

#[test]
fn image_with_single_attribute() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        r#"![foo](/url){width="100pt"}"#,
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "/url".to_owned(),
                title: None,
                alt: "foo".to_owned(),
                attr: Some(ImageAttributes {
                    width: Some("100pt".to_owned()),
                    height: None,
                }),
            })])]
        }
    );
}

#[test]
fn image_with_invalid_attribute() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        r#"![foo](/url){width="100pt" invalid="50pt"}"#,
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "/url".to_owned(),
                title: None,
                alt: "foo".to_owned(),
                attr: Some(ImageAttributes {
                    width: Some("100pt".to_owned()),
                    height: None,
                }),
            })])]
        }
    );
}

#[test]
fn image_with_empty_attributes() {
    let doc = parse_markdown(MarkdownParserState::default(), r#"![foo](/url){}"#).unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Image {
                destination: "/url".to_owned(),
                title: None,
                alt: "foo".to_owned(),
                attr: Some(ImageAttributes {
                    width: None,
                    height: None,
                }),
            })])]
        }
    );
}
