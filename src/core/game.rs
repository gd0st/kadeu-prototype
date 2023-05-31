use crate::core::cards::Card;

pub enum Schedule {
    Linear,
    Random, // TODO: more card types.
}

struct Game<T: Card> {
    cards: Vec<T>,
    schedule: Schedule,
    cursor: &T, // Used to maintain the current card being played.
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

impl<T: Card> Game<T> {
    fn new(cards: Vec<T>, schedule: Schedule) -> Game<T> {
        Game { cards, schedule }
    }

    fn query(&self) -> &String {
        &self.cursor.back()
    }

    fn answer(&self, answer: String) -> bool {
        // cycle to the next card
        // return true or false depending on if they were right or wrong.
        todo!()
    }
}
