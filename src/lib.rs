use std::fmt::Display;



pub mod game {
    use super::Card;
    pub enum Score {
        Accurate,
        Miss, // Maybe a valu that can say /how/ accurate it was.
    }

    pub trait Judge<T: PartialEq> {
        fn score(&self, answer:T) -> Score;
    }
    //Issues right now with trying to get the right value...
    //Might need some input validation on the other side?
    //Seems a bit much to put the onice on the actual gameplay interface...
    impl<T: PartialEq> Judge<T> for Card<T> {
        fn score(&self, answer: T) -> Score {
            match self.value == answer {
                true => Score::Accurate,
                false => Score::Miss
            }
        }
    }

    impl<T: PartialEq> Judge<T> for Card<Vec<T>> {

        fn score(&self, answer: T) -> Score {
            match self.value.iter().any(| value | *value == answer) {
                true => Score::Accurate,
                false => Score::Miss
            }
        }

    }

}


pub trait CardMaker {
    type BACK;
    fn new(front: String, back: Self::BACK) -> Self;
}


#[derive(Clone)]
pub struct Card<T> {
    key: String,
    value: T
}

impl<T: Display> Card<T> {
    pub fn front(&self)-> &String {
        &self.key
    }

}

impl<T: Display> CanDisplay<T> for Card<Vec<T>> {

    fn back(&self) -> String {
        let mut buff: String = String::new();
        for item in self.value.iter() {
            // TODO this is really ugly...
            buff.push_str(item.to_string().as_str());
        }
        buff
    }



}

impl<T: Display> CanDisplay<T> for Card<T> {
    fn back(&self) -> String {
        self.value.to_string()
    }
}


pub trait CanDisplay<T: Display> {
    fn back(&self) -> String;
}

pub fn validate<T: PartialEq>(card: Card<T>, answer:T) -> bool {
    card.value == answer
}

impl<T: PartialEq> CardMaker for Card<T> {
    type BACK = T;
    fn new(front: String, back: T) -> Card<T> {
        Card { key: front, value: back }
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
    }
}
