use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct StandardError;

impl Error for StandardError {}

impl fmt::Display for StandardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something has gone wrong")
    }
}
