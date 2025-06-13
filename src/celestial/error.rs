use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InternalError {
    message: String,
}

impl<'a> Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InternalError {}

impl InternalError {
    pub fn new(reason: &str) -> Self {
        Self { message: String::from(reason) }
    }
}