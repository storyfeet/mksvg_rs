//! # mksvg
//!
//! mksvg is a crate designed for the simple creation of svg files.
//! the "SvgWrite" trait, provides writing methods for the various svg shapes.
//! the "SvgW" implements it, and can be created with anything that implements std::io::Write
//!
//!The "page" mod provides methods for creating pages laid out with "Cards",
//!that is anything that implements the "Card" trait.
//!
//!the "text" mod provides some wrapping utilities for printing multiple lines of text in svg
//!

pub mod args;
pub mod iter;
pub mod macros;
pub mod page;
pub mod path;
pub mod tag;
pub mod text;
pub mod write;

pub use crate::args::{Args, SvgArg};
pub use crate::path::PathD;
pub use crate::tag::Tag;
pub use crate::text::{wrap, Text};
pub use crate::write::{SvgFmt, SvgIO, SvgWrite};
