use crate::decks::Card;
use crate::KCard;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CardBack {
    Word(String),
    Number(i32),
    Float(f32),
    Multiple(Vec<CardBack>),
}
