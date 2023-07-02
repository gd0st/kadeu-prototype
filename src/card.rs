use crate::{KCard, Kadeu, Score};
use serde::{Deserialize, Serialize};

impl KCard<String> {
    fn new(self) -> Box<dyn Kadeu> {
        match self {
            KCard::Simple(key, value) => Box::new(Card::new(key, value)),
        }
    }
}

impl Kadeu for Card<String, String> {
    fn front(&self) -> &String {
        &self.key
    }
    fn back(&self) -> String {
        self.value.to_owned()
    }

    fn score(&self, answer: String) -> Score {
        if self.value == answer {
            Score::Hit
        } else {
            Score::Miss
        }
    }
}
