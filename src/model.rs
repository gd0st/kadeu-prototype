use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CardBack {
    Word(String),
    //Number(i32),
    //Float(f32),
    //Multiple(Vec<CardBack>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Progress {
    successes: u32,
    attempts: u32,
}

// Default standard data model for serialized data formats.
// Concrete types have to KCard interfaces to work properly.
// However, when the details are implemented, the rest of the interface should be able to take care of the rest.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Card<T, U> {
    front: T,
    back: U,
    progress: Option<Progress>,
}

impl<T, U> Card<T, U> {
    pub fn new(front: T, back: U) -> Self {
        Card {
            front,
            back,
            progress: None,
        }
    }

    pub fn success(&mut self) {
        if let Some(mut progress) = self.progress {
            progress.successes += 1;
            progress.attempts += 1;
        } else {
            self.progress = Some(Progress {
                successes: 1,
                attempts: 1,
            })
        }
    }
    pub fn miss(&mut self) {
        todo!()
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
    author: Option<String>,
    // strategy
    description: Option<String>,
    title: String,
    cards: Vec<Card<T, U>>,
}

impl<T, U> Deck<T, U> {
    pub fn cards(&self) -> &Vec<Card<T, U>> {
        &self.cards
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> Option<&String> {
        if let Some(description) = self.description {
            Some(&description)
        } else {
            None
        }
    }
    pub fn author(&self) -> Option<&String> {
        if let Some(author) = self.description {
            Some(&author)
        } else {
            None
        }
    }
}
