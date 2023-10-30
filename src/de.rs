use crate::errors::KadeuError;
use serde::Deserialize;
use serde_json;
use serde_yaml;

pub enum Parser
{
    Json,
    Yaml,
}

impl Parser
{
    pub fn parse<'de, T>(self, text: &'de str) -> Result<T, KadeuError>
    where
        T: Deserialize<'de>,
    {
        let error = KadeuError::ParsingError("Failed to parse text".to_string());

        match self {
            Parser::Json => {
                if let Ok(target) = serde_json::from_str(text) {
                    return Ok(target);
                }
            }
            Parser::Yaml => {
                if let Ok(target) = serde_yaml::from_str(text) {
                    return Ok(target);
                }
            }
        }

        Err(error)
    }
}
