use std::collections::HashMap;
use std::fmt::Display;



pub trait CardMaker {
    type BACK;
    fn new(front: String, back: Self::BACK) -> Self;
}

pub fn validate<T: PartialEq>(input: T, answer: T) -> bool {
    input == answer
}

struct Card<T> {
    front: String,
    back: T
}


impl CardMaker for Card<String> {
    type BACK = String;
    fn new(front: String, back: String) -> Card<String> {
        Card { front, back }
    }
}


//pub struct Deck<T: Card>(Vec<T>);
//impl<T: Card> Deck<T> {
    //pub fn new(cards: Vec<T>) -> Deck<T> {
        //Deck(cards)
    //}
//}

#[cfg(test)]
mod tests {

}
