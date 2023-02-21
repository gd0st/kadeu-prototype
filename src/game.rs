mod lib;
use lib::{Card, Deck};

/// Trait that defines the general
/// flow of a game and how cards are meant to revealed.
trait Game {
    fn new(deck: Deck) -> Self;
}

pub struct Practice {
    deck: Deck,
}

impl Game for Practice {
    fn new(deck: &Deck) -> Practice {
        Practice { deck }
    }
}
