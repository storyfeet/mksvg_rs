//use failure::*;
//use failure_derive::*;
//use anyhow::*;
use thiserror::*;

#[derive(Error, Debug)]
pub enum PageError {
    #[error("Format Error:{}", 0)]
    FmtError(std::fmt::Error),
    #[error("Message{}", 0)]
    SMess(&'static str),
}

impl From<std::fmt::Error> for PageError {
    fn from(e: std::fmt::Error) -> Self {
        PageError::FmtError(e)
    }
}
