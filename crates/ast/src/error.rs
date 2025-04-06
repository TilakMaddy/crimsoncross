use std::fmt::Display;

use crate::cnodes;

#[derive(Debug)]
pub enum Error {
    CNode(cnodes::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "AST Error:")?;
        match self {
            Error::CNode(error) => {
                write!(f, "CIFace - ")?;
                write!(f, "{}", error)?;
            }
        }
        Ok(())
    }
}

pub type Result<T> = core::result::Result<T, Error>;
