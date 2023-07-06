use crate::{Card, CardModifier, Deck, Kadeu, KadeuDeck};
use serde::{Deserialize, Serialize};

impl KadeuDeck for Deck<String, String> {
    fn cards(&self) -> Vec<Box<dyn Kadeu>> {
        let mut cards: Vec<Box<dyn Kadeu>> = vec![];
        for card in self.cards.iter() {
            cards.push(Box::new(card.to_owned()))
        }
        cards
    }
}

#[derive(Debug)]
pub enum DeError {
    ParsingError(String),
}

impl Kadeu for Card<String, String> {
    fn front(&self) -> String {
        self.front.to_owned()
    }

    fn back(&self) -> String {
        self.back.to_owned()
    }

    fn score(&self, answer: String) -> bool {
        let mut target = self.back.to_owned();
        // TODO make a generic Modifier (&T) -> T
        if let Some(modifiers) = &self.modifiers {
            for modfier in modifiers {
                match modfier {
                    CardModifier::IgnoreCase => target = target.to_lowercase(),
                }
            }
        }
        if target == answer {
            true
        } else {
            false
        }
    }
}

pub trait DeckDeserializer<T> {
    fn deserialize(bytes: &str) -> Result<Box<dyn KadeuDeck>, DeError>;
}

pub struct Json;
impl DeckDeserializer<String> for Json {
    fn deserialize(bytes: &str) -> Result<Box<dyn KadeuDeck>, DeError> {
        let deck = serde_json::from_str::<Deck<String, String>>(bytes);
        match deck {
            Err(e) => Err(DeError::ParsingError(e.to_string())),
            Ok(deck) => Ok(Box::new(deck)),
        }
    }
}
