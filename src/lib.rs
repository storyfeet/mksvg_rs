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


extern crate num;
pub mod args;
pub mod write;
pub mod page;
pub mod text;
pub mod path;

pub use write::{SvgIO,SvgFmt,SvgWrite};
pub use args::{Args,SvgArg};
pub use page::Card;
pub use path::PathD;

