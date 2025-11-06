use std::{error::Error, fmt::Display};

use crate::modules::errors::error::HawkTuahError;

#[derive(Debug)]
pub struct DisconnectedError;

impl Display for DisconnectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Disconnected from server")
    }
}

impl HawkTuahError for DisconnectedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}