pub mod de;
pub mod util;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

pub trait Kadeu {
    fn front(&self) -> String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> bool;
}

enum FileFormat {
    Json,
}

struct KFile {
    filepath: String,
    format: FileFormat,
}
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CardModifier {
    IgnoreCase,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Card<U, T> {
    front: U,
    back: T,
    modifiers: Option<Vec<CardModifier>>,
}

#[derive(Deserialize, Serialize)]
pub struct Deck<T> {
    title: String,
    cards: Vec<T>,
}

impl<T: Clone> Deck<T> {}

impl<T> Deck<T> {
    pub fn cards(&self) -> &Vec<T> {
        &self.cards
    }
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }

    fn push(&mut self, card: T) {
        self.cards.push(card)
    }

    fn pop(&mut self) -> Option<T> {
        self.cards.pop()
    }
}

pub mod game {
    use std::collections::btree_map::Iter;

    use super::*;
    #[derive(Debug, Clone)]
    pub enum Mode {
        Practice,
        Test,
        Hardcore,
    }
    pub struct CardGame<T> {
        deck: Deck<T>,
        mode: Option<Mode>,
    }

    impl<T: Kadeu> CardGame<T> {
        pub fn answer(&mut self, answer: String) -> bool {
            let card = self.deck.pop();
            if let Some(card) = card {
                let score = card.score(answer);
                self.deck.push(card);
                return score;
            }
            false
        }
    }

    impl<T> CardGame<T> {
        pub fn new(deck: Deck<T>) -> Self {
            CardGame { deck, mode: None }
        }

        pub fn mode(self, mode: Mode) -> Self {
            CardGame {
                deck: self.deck,
                mode: Some(mode),
            }
        }
        pub fn cards(&self) -> &Vec<T> {
            &self.deck.cards()
        }
    }
}
