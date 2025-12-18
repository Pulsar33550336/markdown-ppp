//! Typst printer for Markdown AST
//!
//! This module provides functionality to render a Markdown Abstract Syntax Tree (AST)
//! into Typst format. The printer supports full CommonMark + GitHub Flavored Markdown
//! features and offers configurable output styles.
//!
//! # Features
//!
//! - **Full AST coverage**: All block and inline elements from CommonMark + GFM
//! - **Proper Typst escaping**: All special characters are properly escaped
//! - **GitHub extensions**: Alerts, task lists, footnotes, strikethrough
//! - **Width control**: Configurable line width for pretty-printing
//!
//! # Basic Usage
//!
//! ```rust
//! use markdown_ppp::ast::*;
//! use markdown_ppp::typst_printer::{render_typst, config::Config};
//!
//! let doc = Document {
//!     blocks: vec![
//!         Block::Heading(Heading {
//!             kind: HeadingKind::Atx(1),
//!             content: vec![Inline::Text("Hello Typst".to_string())],
//!         }),
//!         Block::Paragraph(vec![
//!             Inline::Text("This is ".to_string()),
//!             Inline::Strong(vec![Inline::Text("bold".to_string())]),
//!             Inline::Text(" and ".to_string()),
//!             Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
//!             Inline::Text(" text.".to_string()),
//!         ]),
//!     ],
//! };
//!
//! let typst = render_typst(&doc, Config::default());
//! // Produces:
//! // = Hello Typst
//! //
//! // This is *bold* and _italic_ text.
//! ```
//!
//! # Typst Element Mappings
//!
//! | Markdown          | Typst                                |
//! |-------------------|--------------------------------------|
//! | `# Heading`       | `= Heading`                          |
//! | `**bold**`        | `*bold*`                             |
//! | `*italic*`        | `_italic_`                           |
//! | `~~strike~~`      | `#strike[strike]`                    |
//! | `` `code` ``      | `` `code` ``                         |
//! | `> quote`         | `> quote`                            |
//! | `- list`          | `- item`                             |
//! | `1. ordered`      | `+ item`                             |
//! | `[link](url)`     | `#link("url")[link]`                 |
//! | `![img](url)`     | `#image("url")`                      |
//! | Tables            | `#table(...)`                        |
//! | Code blocks       | ` ``` `                              |

mod block;
pub mod config;
mod inline;
mod table;
pub mod util;

#[cfg(test)]
mod tests;

use crate::ast::*;
use pretty::{Arena, DocBuilder};
use std::{collections::HashMap, rc::Rc};

/// Internal state for Typst rendering
///
/// This structure holds the rendering context including the pretty-printer arena,
/// configuration, and pre-processed indices for footnotes and link definitions.
#[derive(Clone)]
pub(crate) struct State<'a> {
    arena: &'a Arena<'a>,
    config: &'a crate::typst_printer::config::Config,
    /// Mapping of footnote labels to their definitions.
    footnote_definitions: &'a HashMap<String, FootnoteDefinition>,
    /// Mapping of link labels to their definitions.
    link_definitions: &'a HashMap<Vec<Inline>, LinkDefinition>,
    render_with_hash: bool,
}

impl<'a> State<'a> {
    /// Create a new rendering state
    ///
    /// This processes the AST to build indices for footnotes and link definitions,
    /// which are needed for proper cross-referencing during rendering.
    pub fn new(
        arena: &'a Arena<'a>,
        config: &'a crate::typst_printer::config::Config,
        footnote_definitions: &'a HashMap<String, FootnoteDefinition>,
        link_definitions: &'a HashMap<Vec<Inline>, LinkDefinition>,
    ) -> Self {
        Self {
            arena,
            config,
            footnote_definitions,
            link_definitions,
            render_with_hash: true,
        }
    }

    /// Get the footnote definition for a label
    ///
    /// Returns `None` if the footnote is not defined in the document.
    pub fn get_footnote_definition(&self, label: &str) -> Option<&FootnoteDefinition> {
        self.footnote_definitions.get(label)
    }

    /// Get the link definition for a reference link
    ///
    /// Returns `None` if the link reference is not defined in the document.
    pub fn get_link_definition(&self, label: &Vec<Inline>) -> Option<&LinkDefinition> {
        self.link_definitions.get(label)
    }
}

/// Render the given Markdown AST to Typst
///
/// This is the main entry point for Typst rendering. It takes a parsed Markdown
/// document and configuration, then produces Typst source code.
///
/// # Arguments
///
/// * `ast` - The parsed Markdown document as an AST
/// * `config` - Configuration for rendering (width, etc.)
///
/// # Returns
///
/// Typst source code as a string.
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::ast::*;
/// use markdown_ppp::typst_printer::{render_typst, config::Config};
///
/// let doc = Document {
///     blocks: vec![
///         Block::Paragraph(vec![
///             Inline::Text("Visit ".to_string()),
///             Inline::Link(Link {
///                 destination: "https://example.com".to_string(),
///                 title: None,
///                 children: vec![Inline::Text("this link".to_string())],
///             }),
///             Inline::Text(" for more info.".to_string()),
///         ]),
///         Block::List(List {
///             kind: ListKind::Bullet(ListBulletKind::Star),
///             items: vec![ListItem {
///                 task: Some(TaskState::Complete),
///                 blocks: vec![Block::Paragraph(vec![
///                     Inline::Strong(vec![Inline::Text("Bold".to_string())]),
///                     Inline::Text(" item.".to_string()),
///                 ])],
///             }],
///         }),
///     ],
/// };
///
/// let typst = render_typst(&doc, Config::default());
/// // Produces Typst with proper escaping and formatting:
/// // Visit #link("https://example.com")[this link] for more info.
/// //
/// // - [*Bold*] item
/// ```
pub fn render_typst(ast: &Document, config: crate::typst_printer::config::Config) -> String {
    let (footnote_definitions, link_definitions) = get_indices(ast);
    let arena = Arena::new();
    let state = State::new(&arena, &config, &footnote_definitions, &link_definitions);
    let doc = ast.to_doc(&state);

    let mut buf = Vec::new();
    doc.render(config.width, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

/// Internal trait for converting AST nodes to pretty-printer documents
///
/// This trait is implemented by all AST node types and provides the core
/// rendering logic for each element type.
trait ToDoc<'a> {
    /// Convert this AST node to a pretty-printer document
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()>;
}

impl<'a> ToDoc<'a> for Document {
    fn to_doc(&self, state: &'a State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        self.blocks.to_doc(state)
    }
}

/// Extract footnote and link definition indices from the document
///
/// This function performs a pre-processing pass over the AST to:
/// 1. Assign numeric indices to footnote definitions (1, 2, 3, ...)
/// 2. Collect link definitions for reference link resolution
///
/// Returns a tuple of (footnote_index, link_definitions) where:
/// - footnote_index maps footnote labels to their numeric indices
/// - link_definitions maps link labels to their full definitions
fn get_indices(
    ast: &Document,
) -> (
    HashMap<String, FootnoteDefinition>,
    HashMap<Vec<Inline>, LinkDefinition>,
) {
    let mut footnote_definitions = HashMap::new();
    let mut link_definitions = HashMap::new();

    fn process_blocks(
        blocks: &[Block],
        footnote_definitions: &mut HashMap<String, FootnoteDefinition>,
        link_definitions: &mut HashMap<Vec<Inline>, LinkDefinition>,
    ) {
        for block in blocks {
            match block {
                Block::FootnoteDefinition(def) => {
                    footnote_definitions.insert(def.label.clone(), def.clone());
                }
                Block::Definition(def) => {
                    link_definitions.insert(def.label.clone(), def.clone());
                }
                Block::List(list) => {
                    for item in &list.items {
                        process_blocks(&item.blocks, footnote_definitions, link_definitions);
                    }
                }
                Block::BlockQuote(blocks) => {
                    process_blocks(blocks, footnote_definitions, link_definitions);
                }
                Block::GitHubAlert(alert) => {
                    process_blocks(&alert.blocks, footnote_definitions, link_definitions);
                }
                _ => {}
            }
        }
    }

    process_blocks(
        &ast.blocks,
        &mut footnote_definitions,
        &mut link_definitions,
    );

    (footnote_definitions, link_definitions)
}
