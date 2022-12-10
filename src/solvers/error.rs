use std::{fmt::Display, num::ParseIntError, str::Utf8Error};

#[derive(Debug)]
pub struct InputParseError {
    message: String,
}

impl InputParseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    fn from_source<Err: std::error::Error>(err: Err) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl Display for InputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for InputParseError {}

impl From<ParseIntError> for InputParseError {
    fn from(err: ParseIntError) -> Self {
        Self::from_source(err)
    }
}

impl From<Utf8Error> for InputParseError {
    fn from(err: Utf8Error) -> Self {
        Self::from_source(err)
    }
}
