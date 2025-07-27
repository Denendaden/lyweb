#![warn(missing_docs)]

//! Template-based static site generator written for my personal website, <https://lyra.pink>.
//!
//! Currently is only able to fill in templates with the contents of another file.
//! I will add more features only as they are useful to my own website, and perhaps
//! someone will one day find this useful for creating their own.
//!
//! Planned features:
//! - [ ] Navigation bars in templates that disable links to the page you are on
//! - [ ] Conversion from Markdown to HTML to make creating pages easier
//! - [ ] Some kind of blog system that can show lists of recent posts

/// Error handling
pub mod error;

/// Static site generation
pub mod ssg;
