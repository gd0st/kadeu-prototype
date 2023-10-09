use crate::cards::{back::CardBack, cards::Deck};
use std::error::Error;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub enum FormatError {
    ParsingError,
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::ParsingError => write!(f, "Could not parse kadeu set."),
        }
    }
}

impl Error for FormatError {}

pub enum Formatter {
    Json,
}

impl Formatter {
    pub fn parse(self, text: &str) -> Result<Deck<String, CardBack>, Box<dyn Error>> {
        match self {
            Formatter::Json => {
                if let Ok(deck) = serde_json::from_str(text) {
                    Ok(deck)
                } else {
                    todo!()
                }
            }
        }
    }
}
