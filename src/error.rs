use std::fmt;
use std::error::Error as StdError;
#[cfg(features = "serde")]
use serde::ser::Error as SerdeError;

#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    #[cfg(features = "serde")]
    Serialize(String),
    IO(String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(features = "serde")]
impl SerdeError for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error(ErrorKind::Serialize(msg.to_string()))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error(ErrorKind::IO(e.kind().to_string()))
    }
}
