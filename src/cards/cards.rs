use crate::cards::back::CardBack;
use crate::game::{KCard, KDeck, Score};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Card<T, U> {
    front: T,
    back: U,
}

impl<T, U> Card<T, U> {
    pub fn new(front: T, back: U) -> Self {
        Card { front, back }
    }
    pub fn front(&self) -> &T {
        &self.front
    }

    pub fn back(&self) -> &U {
        &self.back
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Deck<T, U> {
    title: String,
    cards: Vec<Card<T, U>>,
}

impl<T, U> Deck<T, U> {
    pub fn cards(&self) -> &Vec<Card<T, U>> {
        &self.cards
    }
}
