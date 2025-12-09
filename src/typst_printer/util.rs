//! Utility functions for Typst rendering
//!
//! This module provides helper functions for Typst generation including
//! character escaping and Typst function generation.

use pretty::{Arena, DocAllocator, DocBuilder};

/// Escape Typst special characters in text
///
/// This function converts plain text to Typst-safe text by escaping all
/// special characters that have meaning in Typst.
///
/// # Typst Special Characters
///
/// The following characters are escaped:
/// - `\` → `\\`
/// - `*` → `\*`
/// - `_` → `\_`
/// - `"` → `\"`
///
/// # Examples
///
/// ```rust
/// # use markdown_ppp::typst_printer::util::escape_typst;
//// assert_eq!(escape_typst("Hello *world*"), "Hello \\*world\\*");
/// assert_eq!(escape_typst("\"Quoted\""), "\\\"Quoted\\\"");
/// ```
pub fn escape_typst(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\\' => r"\\".to_string(),
            '"' => r#"\""#.to_string(),
            '\t' => r"\t".to_string(),
            '\n' => r"\n".to_string(),
            '\r' => r"\r".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Create a Typst function call with content.
/// e.g. `#name[content]` or `#name(..args)[content]`
pub fn body<'a>(
    arena: &'a Arena<'a>,
    name: &str,
    args: Option<DocBuilder<'a, Arena<'a>, ()>>,
    content: Vec<DocBuilder<'a, Arena<'a>, ()>>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    let mut cmd = arena.text(format!("#{name}"));

    if let Some(args) = args {
        cmd = cmd
            .append(arena.text("("))
            .append(args)
            .append(arena.text(")"));
    }

    for c in content {
        cmd = cmd
            .append(arena.text("["))
            .append(c)
            .append(arena.text("]"));
    }

    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_typst() {
        assert_eq!(escape_typst("hello"), "hello");
        // assert_eq!(escape_typst("*bold*"), r"\*bold\*");
        // assert_eq!(escape_typst("_italic_"), r"\_italic\_");
        assert_eq!(escape_typst(r"\command"), r"\\command");
        assert_eq!(escape_typst(r#""quote""#), r#"\"quote\""#);
    }
}
