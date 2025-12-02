/// Configuration for Markdown pretty-printing output.
pub struct Config {
    pub(crate) width: usize,
    pub(crate) spaces_before_list_item: usize,
    pub(crate) empty_line_before_list: bool,
    pub(crate) smart_wrapping: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 80,
            spaces_before_list_item: 1,
            empty_line_before_list: true,
            smart_wrapping: false,
        }
    }
}

impl Config {
    /// Render document with a given width in characters.
    pub fn with_width(self, width: usize) -> Self {
        Self { width, ..self }
    }

    /// Sets the number of spaces to insert before each list item when rendering the AST to Markdown.
    ///
    /// The default is 1 space. According to the GitHub Flavored Markdown specification,
    /// between 0 and 3 spaces before a list marker are allowed:
    /// <https://github.github.com/gfm/#lists>
    ///
    /// # Parameters
    ///
    /// - `spaces`: the number of spaces to insert before each list item (valid range: 0..=3).
    ///
    /// # Returns
    ///
    /// A new printer config instance with `spaces_before_list_item` set to the specified value.
    pub fn with_spaces_before_list_item(self, spaces: usize) -> Self {
        Self {
            spaces_before_list_item: spaces,
            ..self
        }
    }

    /// Sets whether to add an empty line before lists.
    ///
    /// The default is `true`, which means that lists are preceded by an empty line.
    pub fn with_empty_line_before_list(self, tight: bool) -> Self {
        Self {
            empty_line_before_list: tight,
            ..self
        }
    }

    /// Sets whether to use smart wrapping for text.
    ///
    /// The default is `true`, which means that text is wrapped intelligently to avoid
    /// creating unwanted markdown syntax. If set to `false`, text is wrapped without
    /// considering the markdown syntax, which may lead to unintended formatting.
    pub fn with_smart_wrapping(self, smart_wrapping: bool) -> Self {
        Self {
            smart_wrapping,
            ..self
        }
    }
}
