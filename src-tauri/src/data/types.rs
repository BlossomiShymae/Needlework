use std::{error::Error, fmt};

use irelia::LCUError;

#[derive(Debug, Clone)]
pub struct StandardError {
    pub message: String,
}

impl StandardError {
    pub fn new(message: &str) -> StandardError {
        StandardError {
            message: message.to_string(),
        }
    }

    pub fn from_error(error: impl Error) -> StandardError {
        StandardError {
            message: error.to_string(),
        }
    }

    pub fn from_lcu_error(error: LCUError) -> StandardError {
        StandardError {
            message: error.to_string(),
        }
    }
}

impl Error for StandardError {}

impl fmt::Display for StandardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something has gone wrong")
    }
}
