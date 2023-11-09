use crate::decks::model::{self, CardBack};
use crate::decks::{Card, Deck};

pub enum Score {
    Hit,
    Miss,
}

pub trait KDeck {
    fn cards(&self) -> Vec<Box<dyn KCard>>;
}

pub trait KCard {
    fn score(&self, answer: String) -> Score;
}

impl KCard for Card<String, CardBack> {
    fn score(&self, answer: String) -> Score {
        Score::Miss
    }
}
