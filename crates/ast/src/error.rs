use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    CNode(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "AST Error:")?;
        match self {
            Error::CNode(error) => {
                write!(f, "CNode - ")?;
                write!(f, "{}", error)?;
            }
        }
        Ok(())
    }
}

pub type Result<T> = core::result::Result<T, Error>;
