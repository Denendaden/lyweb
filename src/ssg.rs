use crate::error::*;

use std::fs;

/// Stores a webpage, with some methods for modifying and filling out the content.
/// Used as the basis for the LySSG library.
///
/// # Example
/// Creating a LyWebpage from a template and filling the template with content from
/// a file, and then accessing the content of the result:
/// ```rust
/// LyWebpage::read_file("templates/template.html")?
///     .fill_template("content", &fs::read_to_string("www/content.html")?)
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
    /// Creates a new LyWebpage by reading from the file at `filepath`.
    pub fn read_file(filepath: &str) -> Result<LyWebpage, LyError> {
        Ok(LyWebpage {
            contents: fs::read_to_string(filepath)?,
        })
    }

    /// Fills the template with the provided content. Does this by replacing every
    /// instance of `[[key]]` in `self.contents` with `contents`.
    pub fn fill_template(mut self, key: &str, contents: &str) -> Self {
        self.contents = self.contents.replace(&format!("[[{key}]]"), contents);
        self
    }
}
