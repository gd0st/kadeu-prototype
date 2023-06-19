use std::collections::HashMap;
use std::fmt::Display;

pub trait CardMaker {
    type BACK;
    fn new(front: String, back: Self::BACK) -> Self;
    fn make_cards(&self) -> Vec<Card>;
}

#[derive(Clone)]
pub struct Card {
    front: String,
    back: String
}


impl CardMaker for Card {
    type BACK = String;

    fn new(front: String, back: String) -> Self {
        Card { front, back }
    }
    fn make_cards(&self) -> Vec<Card> {
        vec![self.clone()]
    }
}


pub struct ComplexCard {
    front: String,
    back: Vec<String>
}

impl CardMaker for ComplexCard {
    type BACK = Vec<String>;
    fn new(front: String, back: Vec<String>) -> ComplexCard {
        ComplexCard { front, back }
    }
    fn make_cards(&self) -> Vec<Card> {
        self.back
            .iter()
            .map(|back| Card::new(self.front.to_owned(), back.to_owned()))
            .collect()
    }
}




#[cfg(test)]
mod tests {
    use crate::ComplexCard;

    use super::{Card, CardMaker};
    #[test]
    fn make_card() {
        let front: String = "foo".to_string();
        let back: String = "bar".to_string();
        let card: Card = Card::new(
            front,
            back
        );
        assert_eq!("foo".to_string(), card.front)
    }

    #[test]
    fn make_complex_card() {
        let front: String = "foo".to_string();
        let back: Vec<String> = vec!["bar".to_string(), "bazz".to_string()];
        let cards: Vec<Card> = ComplexCard::new(front, back).make_cards();
        for card in cards {
            assert_eq!(card.front, "foo".to_string())
        }

    }
}
