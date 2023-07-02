use serde::{Deserialize, Serialize};
pub mod de_json;

pub trait Kadeu {
    fn front(&self) -> &String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> Result<bool, ()>;
}

pub trait KadeuDeck {
    fn title(&self) -> &String;
    fn cards(&self) -> Vec<Box<dyn Kadeu>>;
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
