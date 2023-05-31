use crate::core::cards::Card;

pub enum Schedule {
    Linear,
    Random, // TODO: more card types.
}

pub struct CardChallenge<T: Card> {
    card: T,
    passed: bool, // if the user passed the challenge?
}

struct Game<T: Card> {
    cards: Vec<T>,
    schedule: Schedule,
}

///
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

impl<'a, T: Card> Game<T> {
    fn new(cards: Vec<T>, schedule: Schedule) -> Game<T> {
        Game { cards, schedule }
    }

    fn query(&self) -> String {
        // still need to find out how to program this properly.
        "foo".to_string()
    }

    fn queue_cards(self) -> Vec<T> {
        self.cards
    }
}
