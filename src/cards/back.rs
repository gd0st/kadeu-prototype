use serde::{Deserialize, Serialize};
use crate::game::{Score, KCard};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CardBack {
    Word(String),
    Number(i32),
    Float(f32),
    Multiple(Vec<CardBack>),
}
