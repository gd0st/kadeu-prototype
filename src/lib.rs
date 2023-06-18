use std::collections::HashMap;

pub enum Value {
    Simple(String),
    Complex(Vec<String>),
}

impl Value {
    fn resolve_answer(self) -> Option<String> {
        match self {
            Value::Simple(s) => Some(s),
            // Not ideal
            Value::Complex(mut ss) => ss.pop(),
        }
    }
}

// Card would need to be an interface if I want to easily spit them
// out of an iterator
pub struct SimpleCard {
    key: String,
    value: String,
}
impl SimpleCard {
    pub fn front(&self) -> &String {
        &self.key
    }
    pub fn back(&self) -> &String {
        &self.value
    }
}
impl Card for SimpleCard {
    fn new(front: String, back: String) -> SimpleCard {
        SimpleCard {
            key: front,
            value: back,
        }
    }
    // generic new function, but handeld by a factory?
    fn dynamic_new(front: String, back: Value) -> SimpleCard {
        match back {
            Value::Simple(string) => SimpleCard {
                key: front,
                value: string,
            },
            Value::Complex(mut backs) => {
                let value: String = backs.pop().unwrap_or("foobar".to_string());
                SimpleCard { key: front, value }
            }
        }
    }
}

pub trait Card {
    fn new(front: String, back: String) -> Self;
    fn dynamic_new(front: String, back: Value) -> Self;
}
pub struct Deck<T: Card>(Vec<T>);

impl<T: Card> Deck<T> {
    pub fn new(cards: Vec<T>) -> Deck<T> {
        Deck(cards)
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Deck, SimpleCard};
    #[test]
    fn make_simple_card() {
        let key = "foo".to_string();
        let value = "bar".to_string();
        let card = SimpleCard::new(key, value);
        assert_eq!(card.0, String::from("foo"));
        assert_eq!(card.1, String::from("bar"));
    }

    #[test]
    fn make_deck() {
        let key = "foo".to_string();
        let value = "bar".to_string();
        let card = SimpleCard::new(key, value);
        let _ = Deck(vec![card]);
    }
}
