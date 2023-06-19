use std::collections::HashMap;
use std::fmt::Display;




pub trait CardMaker {
    type BACK;
    fn new(front: String, back: Self::BACK) -> Self;
    fn make_cards(&self) -> Vec<Card<String>>;
}

pub trait KadeuCard {
    fn front(&self) -> &String;
    fn back(&self) -> &String;
}

#[derive(Clone)]
pub struct Card<T> {
    key: String,
    value: T
}

impl<T: Display> Card<T> {
    pub fn front(&self) -> &String {
        &self.key
    }
    pub fn back(&self) -> &T {
        &self.value
    }
}

pub fn validate<T: PartialEq>(card: Card<T>, answer:T) -> bool {
    card.value == answer
}

impl CardMaker for Card<String> {
    type BACK = String;
    fn new(front: String, back: String) -> Card<String> {
        Card { key: front, value: back }
    }

    fn make_cards(&self) -> Vec<Card<String>>{
        vec![self.clone()]
    }
}

impl CardMaker for Card<Vec<String>> {
    type BACK = Vec<String>;
    fn new(front: String, back: Vec<String>) -> Card<Vec<String>>{
        Card { key: front, value: back}
    }

    fn make_cards(&self) -> Vec<Card<String>> {
        self.value
            .iter()
            .map(|back| Card::new(self.key.to_owned(), back.to_owned()))
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::{Card, CardMaker};

    #[test]
    fn make_card() {
        let front: String = "foo".to_string();
        let back: String = "bar".to_string();
        let card: Card<String> = Card::new(front, back);

        assert_eq!(card.key, "foo".to_string());
        assert_eq!(card.value, "bar".to_string());
    }

    #[test]
    fn make_complex_card() {
        let front: String = "foo".to_string();
        let back: Vec<String> = vec!["bar".to_string(), "bazz".to_string()];
        let card: Card<Vec<String>> = Card::new(front, back);

        let cards = card.make_cards();

        for card in cards {
            assert_eq!(card.key, "foo".to_string());
        }
    }
}
