use std::fmt;
use std::error::Error as StdError;
use std::str::Utf8Error;
// #[cfg(features = "serde")]
use serde::ser::Error as SeError;
use serde::de::Error as DeError;

#[derive(Debug)]
pub struct Error(pub ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    // #[cfg(features = "serde")]
    Serialize(String),
    Deserialize(String),
    IO(String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// #[cfg(features = "serde")]
impl SeError for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error(ErrorKind::Serialize(msg.to_string()))
    }
}

// #[cfg(features = "serde")]
impl DeError for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error(ErrorKind::Deserialize(msg.to_string()))
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error(ErrorKind::Deserialize(e.to_string()))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error(ErrorKind::IO(e.kind().to_string()))
    }
}
