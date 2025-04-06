use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    CConst(String),
}

pub type Result<T> = core::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CConst(string) => writeln!(f, "CConst: {}", string),
        }
    }
}
