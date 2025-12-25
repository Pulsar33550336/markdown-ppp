//! Markdown parser for CommonMark + GitHub Flavored Markdown (GFM)
//!
//! This module provides a comprehensive parser for Markdown documents following the
//! CommonMark specification with GitHub Flavored Markdown extensions. The parser
//! converts raw Markdown text into a fully-typed Abstract Syntax Tree (AST).
//!
//! # Features
//!
//! - **CommonMark compliance**: Full support for CommonMark 1.0 specification
//! - **GitHub extensions**: Tables, task lists, strikethrough, autolinks, footnotes, alerts
//! - **Configurable parsing**: Control which elements to parse, skip, or transform
//! - **Custom parsers**: Register custom block and inline element parsers
//! - **Error handling**: Comprehensive error reporting with nom-based parsing
//!
//! # Basic Usage
//!
//! ```rust
//! use markdown_ppp::parser::{parse_markdown, MarkdownParserState};
//!
//! let state = MarkdownParserState::new();
//! let input = "# Hello World\n\nThis is **bold** text.";
//!
//! match parse_markdown(state, input) {
//!     Ok(document) => {
//!         println!("Parsed {} blocks", document.blocks.len());
//!     }
//!     Err(err) => {
//!         eprintln!("Parse error: {:?}", err);
//!     }
//! }
//! ```
//!
//! # Configuration
//!
//! The parser behavior can be extensively customized using configuration:
//!
//! ```rust
//! use markdown_ppp::parser::{MarkdownParserState, config::*};
//!
//! let config = MarkdownParserConfig::default()
//!     .with_block_thematic_break_behavior(ElementBehavior::Skip)
//!     .with_inline_emphasis_behavior(ElementBehavior::Parse);
//!
//! let state = MarkdownParserState::with_config(config);
//! ```

mod blocks;

/// Configuration options for Markdown parsing behavior.
pub mod config;
mod inline;
mod link_util;
mod util;

use crate::ast::Document;
use crate::parser::config::MarkdownParserConfig;
use nom::{
    branch::alt,
    character::complete::{line_ending, space1},
    combinator::eof,
    multi::many0,
    sequence::terminated,
    Parser,
};
use std::rc::Rc;
use std::borrow::Cow;

/// Parser state containing configuration and shared context
///
/// This structure holds the parser configuration and provides shared state
/// during the parsing process. It's designed to be cloned cheaply using
/// reference counting for the configuration.
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::parser::{MarkdownParserState, config::MarkdownParserConfig};
///
/// // Create with default configuration
/// let state = MarkdownParserState::new();
///
/// // Create with custom configuration
/// let config = MarkdownParserConfig::default();
/// let state = MarkdownParserState::with_config(config);
/// ```
/// Note: This struct is marked `#[non_exhaustive]` to allow adding new fields
/// in future versions without breaking existing code.
#[non_exhaustive]
pub struct MarkdownParserState {
    /// The parser configuration (reference-counted for efficient cloning)
    pub config: Rc<MarkdownParserConfig>,
    /// Whether we are parsing content extracted from a container block (list item, blockquote, etc.)
    /// When true, fenced code blocks should not strip additional indentation from their content.
    /// This field is for internal use only.
    pub(crate) is_nested_block_context: bool,

    /// The stack of containers that are currently being parsed.
    /// This is used to prevent self-nesting.
    pub(crate) containers: Vec<String>,
}

impl MarkdownParserState {
    /// Create a new parser state with default configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::parser::MarkdownParserState;
    ///
    /// let state = MarkdownParserState::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new parser state with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The parser configuration to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::parser::{MarkdownParserState, config::MarkdownParserConfig};
    ///
    /// let config = MarkdownParserConfig::default();
    /// let state = MarkdownParserState::with_config(config);
    /// ```
    pub fn with_config(config: MarkdownParserConfig) -> Self {
        Self {
            config: Rc::new(config),
            is_nested_block_context: false,
            containers: Vec::new(),
        }
    }

    /// Create a nested parser state for parsing content extracted from container blocks
    ///
    /// This method creates a new state that shares the same configuration but marks
    /// the parsing context as nested. This prevents double-stripping of indentation
    /// when parsing fenced code blocks inside list items, blockquotes, etc.
    pub(crate) fn nested(&self) -> Self {
        Self {
            config: self.config.clone(),
            is_nested_block_context: true,
            containers: self.containers.clone(),
        }
    }
}

impl Default for MarkdownParserState {
    fn default() -> Self {
        Self::with_config(MarkdownParserConfig::default())
    }
}

/// Parse a Markdown string into an Abstract Syntax Tree (AST)
///
/// This is the main entry point for parsing Markdown text. It processes the input
/// according to the CommonMark specification with GitHub Flavored Markdown extensions,
/// returning a fully-typed AST that can be manipulated, analyzed, or rendered.
///
/// # Arguments
///
/// * `state` - Parser state containing configuration options
/// * `input` - The Markdown text to parse
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(Document)` - Successfully parsed AST document
/// - `Err(nom::Err)` - Parse error with position and context information
///
/// # Examples
///
/// Basic parsing:
/// ```rust
/// use markdown_ppp::parser::{parse_markdown, MarkdownParserState};
///
/// let state = MarkdownParserState::new();
/// let result = parse_markdown(state, "# Hello\n\nWorld!");
///
/// match result {
///     Ok(doc) => println!("Parsed {} blocks", doc.blocks.len()),
///     Err(e) => eprintln!("Parse error: {:?}", e),
/// }
/// ```
///
/// With custom configuration:
/// ```rust
/// use markdown_ppp::parser::{parse_markdown, MarkdownParserState};
/// use markdown_ppp::parser::config::*;
///
/// let config = MarkdownParserConfig::default()
///     .with_block_thematic_break_behavior(ElementBehavior::Skip);
/// let state = MarkdownParserState::with_config(config);
///
/// let doc = parse_markdown(state, "---\n\nContent").unwrap();
/// ```
///
/// # Errors
///
/// Returns a parse error if the input contains invalid Markdown syntax
/// that cannot be recovered from. Most malformed Markdown is handled
/// gracefully according to CommonMark's error handling rules.
pub fn parse_markdown(
    state: MarkdownParserState,
    input: &str,
) -> Result<Document, nom::Err<nom::error::Error<String>>> {
    let transformed_input =
        if let Some(inline_macro_replacer) = &state.config.inline_macro_replacer {
            let mut replacer = inline_macro_replacer.borrow_mut();
            let mut result = String::new();
            let mut last_pos = 0;

            while let Some(start_pos) = input[last_pos..].find("{{") {
                let absolute_start = last_pos + start_pos;
                let mut balance = 1;
                let mut current_scan_pos = absolute_start + 2;
                let mut end_pos = None;

                while let Some(next_marker_pos) = input[current_scan_pos..].find(|c| c == '{' || c == '}') {
                    let absolute_marker_pos = current_scan_pos + next_marker_pos;
                    if input.get(absolute_marker_pos..absolute_marker_pos + 2) == Some("{{") {
                        balance += 1;
                        current_scan_pos = absolute_marker_pos + 2;
                    } else if input.get(absolute_marker_pos..absolute_marker_pos + 2) == Some("}}") {
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
                    result.push_str(&input[last_pos..absolute_start]);
                    let content = &input[absolute_start + 2..absolute_end];
                    let replacement = (replacer)(content.trim());
                    result.push_str(&replacement);
                    last_pos = absolute_end + 2;
                } else {
                    // No matching end found, stop processing
                    break;
                }
            }

            result.push_str(&input[last_pos..]);
            Cow::Owned(result)
        } else {
            Cow::Borrowed(input)
        };

    let empty_lines = many0(alt((space1, line_ending)));
    let mut parser = terminated(
        many0(crate::parser::blocks::block(Rc::new(state))),
        (empty_lines, eof),
    );
    let result = parser.parse(&transformed_input);

    match result {
        Ok((_, blocks)) => {
            let blocks = blocks.into_iter().flatten().collect();
            Ok(Document { blocks })
        }
        Err(nom::Err::Error(e)) => Err(nom::Err::Error(nom::error::Error {
            input: e.input.to_string(),
            code: e.code,
        })),
        Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(nom::error::Error {
            input: e.input.to_string(),
            code: e.code,
        })),
        Err(nom::Err::Incomplete(needed)) => Err(nom::Err::Incomplete(needed)),
    }
}
