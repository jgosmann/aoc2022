use std::fmt::Display;

#[derive(Debug)]
pub struct InputParseError {
    message: String,
}

impl InputParseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for InputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for InputParseError {}
