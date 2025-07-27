use crate::error::*;

use regex::Regex;

use std::{cell::OnceCell, fs, path::Path};

/// Stores a webpage, with some methods for modifying and filling out the content.
/// Used as the basis for the LySSG library.
///
/// # Example
/// Creating a LyWebpage from a template and filling the template with content from
/// a file, and then accessing the content of the result:
/// ```rust
/// LyWebpage::from_file("templates/main.html")?
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

    /// Resolves statements of the form `[[IF [path] ... ELSE ...]]`
    ///
    /// # Example
    /// Creating a navigation bar that deactivates links for the page you are
    /// already on:
    /// ```html
    /// [[IF / <b>Home</b> ELSE <a href="/">Home</a>]]
    /// [[IF blog <b>Blog</b> ELSE <a href="/blog">Blog</a>]]
    /// ```
    /// Calling this on the backend:
    /// ```rust
    /// LyWebpage::from_file("templates/main.html")?
    ///     .resolve_ifs("blog")? // if /blog was requested
    ///     .contents
    /// ```
    pub fn resolve_ifs(mut self, expr: &str) -> Result<Self, LyError> {
        // use a OnceCell to cache the compiled regex and avoid recompiling
        let re_cell = OnceCell::new();

        let re = match re_cell.get() {
            Some(r) => r,
            None => {
                let r = Regex::new(r#"(?s)\[\[\s*IF\s+(\S+)(.*?)ELSE\s+(.*?)\]\]"#)?;
                let _ = re_cell.set(r);
                re_cell.get().ok_or(LyError::TemplatingError)?
            }
        };

        // store byte offsets of regex groups
        let mut locs = re.capture_locations();

        // macro to make accessing values in locs a bit easier
        macro_rules! loc {
            ( $i:expr ) => {
                locs.get($i).ok_or(LyError::TemplatingError)?
            };
        }

        // create a new String to replace the current self.contents
        let mut s = String::new();
        let mut i = 0;

        while let Some(_) = re.captures_read_at(&mut locs, &self.contents, i) {
            s += &self.contents[i..loc!(0).0];
            if expr == &self.contents[loc!(1).0..loc!(1).1] {
                // path matches IF branch; use that
                s += &self.contents[loc!(2).0..loc!(2).1];
            } else {
                // path matches ELSE branch; use that
                s += &self.contents[loc!(3).0..loc!(3).1];
            }
            i = loc!(0).1;
        }

        s += &self.contents[i..];

        self.contents = s;

        Ok(self)
    }
}
