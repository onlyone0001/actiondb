use std::io;
use serde::json;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Deser(DeserError)
}

#[derive(Debug)]
pub enum DeserError {
    JSON(json::Error)
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<json::Error> for DeserError {
    fn from(error: json::Error) -> DeserError {
        DeserError::JSON(error)
    }
}

impl From<DeserError> for Error {
    fn from(error: DeserError) -> Error {
        Error::Deser(error)
    }
}