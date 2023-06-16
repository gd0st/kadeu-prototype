use std::collections::HashMap;

// Card would need to be an interface if I want to easily spit them
// out of an iterator
struct SimpleCard(String, String);
impl Card for SimpleCard {
    fn new(front: String, back: String) -> SimpleCard {
        SimpleCard(front, back)
    }
}
trait Card {
    fn new(front: String, back: String) -> Self;
}
struct Deck<T: Card>(Vec<T>);

#[cfg(test)]
mod tests {
    use super::{Card, Deck, SimpleCard};
    #[test]
    fn make_simple_card() {
        let key = "foo".to_string();
        let value = "bar".to_string();
        let card = SimpleCard(key, value);
        assert_eq!(card.0, String::from("foo"));
        assert_eq!(card.1, String::from("bar"));
    }

    #[test]
    fn make_deck() {
        let key = "foo".to_string();
        let value = "bar".to_string();
        let card = SimpleCard(key, value);
        let _ = Deck(vec![card]);
    }
}
