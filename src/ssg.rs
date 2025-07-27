use crate::error::*;

use std::{fs, path::Path};

/// Stores a webpage, with some methods for modifying and filling out the content.
/// Used as the basis for the LySSG library.
///
/// # Example
/// Creating a LyWebpage from a template and filling the template with content from
/// a file, and then accessing the content of the result:
/// ```rust
/// LyWebpage::from_file("templates/template.html")?
///     .fill_from_file("content", "www/content.html")?
///     .contents
/// ```
/// In `templates/template.html`:
/// ```html
/// <div id="contents">
///     [[content]]
/// </div>
/// ```
pub struct LyWebpage {
    /// The contents of the webpage, typically stored as HTML.
    pub contents: String,
}

impl LyWebpage {
    /// Creates a new `LyWebpage` from a provided `&str`.
    pub fn from_str(s: &str) -> Self {
        Self { contents: s.to_string() }
    }

    /// Creates a new `LyWebpage` by reading from the file at `filepath`.
    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Result<Self, LyError> {
        Ok(Self::from_str(&fs::read_to_string(filepath)?))
    }

    /// Fills the template with the provided content. Does this by replacing every
    /// instance of `[[key]]` in `self.contents` with `contents`.
    pub fn fill_with_str(mut self, key: &str, contents: &str) -> Self {
        self.contents = self.contents.replace(&format!("[[{key}]]"), contents);
        self
    }

    /// Fills the template with the content of the given file.
    pub fn fill_from_file<P: AsRef<Path>>(self, key: &str, filepath: P) -> Result<Self, LyError> {
        Ok(self.fill_with_str(key, &fs::read_to_string(filepath)?))
    }
}
