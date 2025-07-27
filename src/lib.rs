#![warn(missing_docs)]

//! Template-based static site generator written for my personal website, <https://lyra.pink>.
//!
//! Currently is only able to fill in templates with the contents of another file.
//! I will add more features only as they are useful to my own website, and perhaps
//! someone will one day find this useful for creating their own.

/// Error handling
pub mod error;

/// Static site generation
pub mod ssg;
