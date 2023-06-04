use crate::cards::{Card, Challenge, ChallengeInterface};

enum Answer {
    Simple(String),
    Complex(Vec<String>),
}

// TODO impl Schedule for each type of schedule?
//pub enum Schedule {
//Linear,
//Random, // TODO: more card types.
//}

pub struct CardChallenge<T: Card> {
    card: T,
    passed: bool, // if the user passed the challenge?
}

struct Game<T: Card> {
    cards: Vec<T>,
}

/// ! src/core/game.rs
/// Ideally the gameplay loop would be as such:
///
/// Game Configurations:
/// The user selects a deck they would like to practice
/// The use selects the scheduling algorithm they would like to use.
///
/// Gameplay loop:
/// The user is prompted with a question.
/// The user answers the question with whatever given input.
/// The user is notified if the answer was correct or false.
/// The game ends when the game engine decides the user has answered enough questions.
