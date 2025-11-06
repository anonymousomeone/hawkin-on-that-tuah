use std::{error::Error, fmt::Display};
use crate::modules::errors::error::HawkTuahError;

#[derive(Debug)]
pub struct ConnectionError {
    pub details: String,
}
impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection error: {}", self.details)
    }
}

impl HawkTuahError for ConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}