pub mod cards;
pub mod config;
pub mod errors;
pub mod game;
use std::fs;

pub mod util;
use cards::cards::Deck;
pub mod de;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::cards::{back::CardBack, cards::Card};
use crate::game::{KCard, KDeck, Score, Tally};

pub fn load_deck(filepath: &str) -> Result<Deck<String, CardBack>, errors::KadeuError> {
    println!("{}", &filepath);
    let text = fs::read_to_string(filepath).expect("Loaded file text.");
    let parser = de::FileParser::new(text.as_str(), de::FileFormat::Json);
    parser.parse::<String, CardBack>()
}

impl KCard for Card<String, CardBack> {
    fn front(&self) -> String {
        self.front().to_string()
    }
    fn back(&self) -> String {
        self.back().to_string()
    }
    fn score(&self, answer: String) -> Score {
        todo!()
    }
}

impl Display for CardBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CardBack {
    fn score(&self, answer: String) -> Score {
        match self {
            // TODO I want to split these into answer handlers
            CardBack::Number(target) => {
                if let Ok(answer) = answer.parse::<i32>() {
                    if answer == *target {
                        return vec![Tally::Hit].into();
                    }
                }
                vec![Tally::Miss].into()
            }
            CardBack::Word(target) => {
                if answer == *target {
                    vec![Tally::Hit].into()
                } else {
                    vec![Tally::Miss].into()
                }
            }
            _ => {
                todo!()
            }
        }
    }
}
