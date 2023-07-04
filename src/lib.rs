pub mod util;

pub trait Kadeu {
    fn front(&self) -> String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> bool;
}

pub trait KadeuDeck {
    fn title(&self) -> &String;
    fn cards(&self) -> Vec<Box<dyn Kadeu>>;
}

pub mod de {

    use crate::card::Deck;
    #[derive(Debug)]
    pub enum DeError {
        ParsingError(String),
    }

    pub trait DeckDeserializer<T> {
        fn deserialize(bytes: &str) -> Result<Deck<T>, DeError>;
    }
    pub mod json {
        use super::DeError;
        use super::DeckDeserializer;
        pub struct Json;
        impl DeckDeserializer<String> for Json {
            fn deserialize(bytes: &str) -> Result<crate::card::Deck<String>, DeError> {
                match serde_json::from_str(bytes) {
                    Err(e) => Err(DeError::ParsingError(e.to_string())),
                    Ok(deck) => Ok(deck),
                }
            }
        }
    }
}

pub mod card {
    use crate::Kadeu;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "lowercase")]
    enum Compliance {
        Min(f32),
        Max(f32),
        Strict,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(untagged)]
    enum Side<T> {
        Simple(T),
    }

    #[derive(Deserialize, Serialize)]
    pub struct Card<T> {
        front: Side<T>,
        back: Side<T>,
    }

    impl Into<String> for Side<String> {
        fn into(self) -> String {
            match self {
                Side::Simple(s) => s.to_owned(),
            }
        }
    }
    #[derive(Deserialize, Serialize)]
    pub struct Deck<T> {
        title: String,
        cards: Vec<Card<T>>,
    }

    impl Deck<String> {
        pub fn cards(&self) -> Vec<&Card<String>> {
            self.cards.iter().collect()
        }

        pub fn title(&self) -> &String {
            &self.title
        }
    }

    impl Kadeu for Card<String> {
        fn front(&self) -> String {
            match &self.front {
                Side::Simple(s) => s.to_owned(),
            }
        }
        fn back(&self) -> String {
            match &self.back {
                Side::Simple(s) => s.to_owned(),
            }
        }
        fn score(&self, answer: String) -> bool {
            if self.back() == answer {
                true
            } else {
                false
            }
        }
    }
}

mod game {
    use super::*;
    enum Mode {
        Practice,
        Test,
        Hardcore,
    }
    struct Game {
        cards: Vec<Box<dyn Kadeu>>,
        mode: Mode,
    }

    impl Game {
        fn new(cards: Vec<Box<dyn Kadeu>>, mode: Mode) -> Game {
            Game { cards, mode }
        }
    }

    impl Iterator for Game {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            match self.cards.pop() {
                Some(card) => {
                    let prompt = card.front().to_owned();
                    self.cards.push(card);
                    Some(prompt)
                }
                None => None,
            }
        }
    }
}
