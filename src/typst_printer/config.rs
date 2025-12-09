//! Configuration for Typst rendering
//!
//! This module provides configuration options to customize the Typst output
//! style and format.

/// Configuration for Typst rendering
///
/// This struct controls various aspects of how the Markdown AST is converted
/// to Typst. Use the builder methods to customize the output style.
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::typst_printer::config::*;
///
/// // Default configuration
/// let config = Config::default();
///
/// // Custom configuration
/// let config = Config::default()
///     .with_width(120);
/// ```
pub struct Config {
    pub(crate) width: usize,
}

impl Default for Config {
    /// Create a default configuration
    ///
    /// Default settings:
    /// - Width: 80 characters
    fn default() -> Self {
        Self { width: 80 }
    }
}

impl Config {
    /// Set the line width for pretty-printing
    ///
    /// Controls how the pretty-printer wraps long lines. This affects the
    /// formatting of the generated Typst, not the content itself.
    ///
    /// # Arguments
    ///
    /// * `width` - Maximum line width in characters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_ppp::typst_printer::config::Config;
    ///
    /// let config = Config::default().with_width(120);
    /// ```
    pub fn with_width(self, width: usize) -> Self {
        Self { width, ..self }
    }
}
