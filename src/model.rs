use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CardBack {
    Word(String),
    //Number(i32),
    //Float(f32),
    //Multiple(Vec<CardBack>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Deck<T, U> {
    author: Option<String>,
    // strategy
    description: Option<String>,
    title: String,
    cards: Vec<Card<T, U>>,
}

impl<T, U> Deck<T, U> {
    pub fn cards(&self) -> Vec<&Card<T, U>> {
        self.cards.iter().collect()
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn author(&self) -> Option<&String> {
        self.author.as_ref()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Card<T, U> {
    front: T,
    back: U,
}

impl<T, U> Card<T, U> {
    pub fn new(front: T, back: U) -> Self {
        Card { front, back }
    }
    pub fn back(&self) -> &U {
        &self.back
    }
    pub fn front(&self) -> &T {
        &self.front
    }
}
