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

#[cfg(test)]
mod tests {
    use crate::{error::*, ssg::*};

    // helper function to avoid problems with whitespace when checking test results
    fn remove_whitespace(s: &mut String) {
        s.retain(|c| !c.is_whitespace())
    }

    #[test]
    fn md_works() {
        let mut page = LyWebpage::from_str("<div>[[markdown]]</div>")
            .fill_from_md_str("markdown", "# HEADER", true)
            .contents;
        remove_whitespace(&mut page);

        assert_eq!(page, "<div><h1>HEADER</h1></div>");
    }
}
