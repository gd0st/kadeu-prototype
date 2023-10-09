use crate::cards::back::CardBack;
use crate::cards::cards::Deck;
use crate::errors::KadeuError;
use serde::Deserialize;
use serde_yaml;

pub enum FileFormat {
    Json,
    Yaml,
}

pub struct FileParser<'a> {
    file_format: FileFormat,
    text: &'a str,
}

impl<'de> FileParser<'de> {
    pub fn new(text: &'de str, file_format: FileFormat) -> Self {
        FileParser { text, file_format }
    }
    pub fn parse<T, U>(&self) -> Result<Deck<T, U>, KadeuError>
    where
        T: Deserialize<'de>,
        U: Deserialize<'de>,
    {
        let mut deck: Option<Deck<T, U>> = None;

        //TODO I am in hell right now.
        match self.file_format {
            FileFormat::Json => Json::parse(self.text),
            FileFormat::Yaml => Yaml::parse(self.text),
            _ => Err(KadeuError::ParsingError(
                "File Format missing parsing engine".to_string(),
            )),
        }
    }
}

pub trait Parser {
    fn parse<'de, T, U>(text: &'de str) -> Result<Deck<T, U>, KadeuError>
    where
        T: Deserialize<'de>,
        U: Deserialize<'de>;
}

struct Json;
impl Parser for Json {
    fn parse<'de, T, U>(text: &'de str) -> Result<Deck<T, U>, KadeuError>
    where
        T: Deserialize<'de>,
        U: Deserialize<'de>,
    {
        match serde_json::from_str(text) {
            Ok(deck) => Ok(deck),
            Err(msg) => Err(KadeuError::ParsingError(msg.to_string())),
        }
    }
}

struct Yaml;
impl Parser for Yaml {
    fn parse<'de, T, U>(text: &'de str) -> Result<Deck<T, U>, KadeuError>
    where
        T: Deserialize<'de>,
        U: Deserialize<'de>,
    {
        match serde_yaml::from_str(text) {
            Ok(deck) => Ok(deck),
            Err(e) => Err(KadeuError::ParsingError(e.to_string())),
        }
    }
}
