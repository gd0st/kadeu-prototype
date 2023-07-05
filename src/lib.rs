pub mod de;
pub mod util;
use serde::{Deserialize, Serialize};

pub trait Kadeu {
    fn front(&self) -> String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> bool;
}

pub trait KadeuDeck {
    fn cards(&self) -> Vec<Box<dyn Kadeu>>;
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Card<U, T> {
    front: U,
    back: T,
}

#[derive(Deserialize, Serialize)]
struct Deck<String, T> {
    title: String,
    cards: Vec<Card<String, T>>,
}

pub mod game {
    use super::*;
    #[derive(Debug, Clone)]
    pub enum Mode {
        Practice,
        Test,
        Hardcore,
    }
    pub struct Game {
        cards: Vec<Box<dyn Kadeu>>,
        mode: Mode,
    }

    impl Game {
        pub fn new(cards: Vec<Box<dyn Kadeu>>, mode: Mode) -> Game {
            Game { cards, mode }
        }
        pub fn answer(&mut self, answer: String) -> bool {
            let card = self.cards.pop();
            if let Some(card) = card {
                let score = card.score(answer);
                self.cards.push(card);
                return score;
            }
            false
        }
        pub fn cards(&self) -> &Vec<Box<dyn Kadeu>> {
            &self.cards
        }
    }
}
