use serde::Deserialize;

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

// FIXME change name to Parser
pub enum Formatter {
    Json,
}

impl Formatter {
    pub fn parse<'de, T, U>(self, text: &'de str) -> Result<Deck<T, U>, Box<dyn Error>>
    where
        T: Deserialize<'de>,
        U: Deserialize<'de>,
    {
        match self {
            Formatter::Json => {
                if let Ok(deck) = serde_json::from_str(text) {
                    Ok(deck)
                } else {
                    // TODO need to be able to parse from yaml aswell.
                    todo!()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE_JSON: &str = r#"
{
  "title": "foo",
  "cards": [
    { "front": "bar", "back": "baz" }
]
}

"#;
    use super::*;
    #[test]
    fn parse_simple_deck() {
        let formatter = Formatter::Json;
        let deck: Deck<String, CardBack> = formatter.parse(EXAMPLE_JSON).unwrap();
        assert_eq!(*deck.title(), "foo".to_string())
    }
    fn parse_yaml() {
        //TODO
    }
}
