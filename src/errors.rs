use std::{error::Error, fmt};
#[derive(Debug, Clone)]
pub enum KadeuError {
    ParsingError(String),
}
impl Error for KadeuError {}
impl fmt::Display for KadeuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KadeuError::ParsingError(msg) => write!(f, "{}", msg),
        }
    }
}
