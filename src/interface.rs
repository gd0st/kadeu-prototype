use crate::de;
use crate::errors::KadeuError;
use crate::model::{Card, CardBack, Deck, Progress};
use serde::Deserialize;
use std::fmt::Display;

pub enum Score {
    Hit,
    Miss,
}

pub trait Judge {
    fn validate(&self, answer: &String) -> Score;
}

pub trait KCard {
    fn prompt(&self) -> String;
    fn score(&self, answer: &String) -> Score;
}

impl<T, U> KCard for Card<T, U>
where
    T: Display,
    U: Judge,
{
    fn prompt(&self) -> String {
        self.prompt().to_string()
    }
    fn score(&self, answer: &String) -> Score {
        self.back().validate(answer)
    }
}

impl Judge for CardBack {
    fn validate(&self, answer: &String) -> Score {
        match self {
            CardBack::Word(target) => {
                if target == answer {
                    return Score::Hit;
                }
                Score::Miss
            }
        }
    }
}

trait Sequence<T> {
    fn next(&self, cards: Vec<&T>) -> &T;
}
struct Controller<T, U> {
    cards: Vec<T>,
    discard: Vec<T>,
    sequence: U,
}

impl<T, U> Controller<T, U>
where
    T: KCard,
    U: Strategy<T>,
{
    fn new(cards: Vec<T>, strategy: U) -> Self {
        Controller {
            cards,
            sequence,
            discard: vec![],
        }
    }

    fn next(&self) -> &T {
        self.strategy.next(self.cards.iter().collect())
    }
    fn input(&self, answer: &String) -> Score {
        todo!()
    }
}
