use crate::decks::{model::CardBack, Card, Deck};
use crate::errors::KadeuError;
use serde::Deserialize;
use serde_json;
use serde_yaml;

pub enum Parser<'de, T>
where
    T: Deserialize<'de>,
{
    Json(&'de str, T),
    Yaml(&'de str, T),
}

impl<'de, T> Parser<'de, T>
where
    T: Deserialize<'de>,
{
    pub fn parse(self) -> Result<T, KadeuError>
    where
        T: Deserialize<'de>,
    {
        let error = KadeuError::ParsingError("Failed to parse text".to_string());

        match self {
            Parser::Json(text, target) => {
                if let Ok(target) = serde_json::from_str(text) {
                    return Ok(target);
                }
            }
            Parser::Yaml(text, target) => {
                if let Ok(target) = serde_yaml::from_str(text) {
                    return Ok(target);
                }
            }
        }

        Err(error)
    }
}
