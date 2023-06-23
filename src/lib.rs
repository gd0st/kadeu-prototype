use std::{backtrace::Backtrace, fmt::Display};

use cards::MakeCard;

pub enum Compliancy {
    Strict,
    Minimum(f64),
    Maximum(f64),
}
impl Compliancy {
    fn passed(self, accuracy: f64) -> bool {
        match self {
            Compliancy::Strict => accuracy == 100 as f64,
            Compliancy::Maximum(max) => accuracy < max,
            Compliancy::Minimum(min) => accuracy > min,
        }
    }
}

pub enum Score {
    Accurate,
    Miss, // Maybe a valu that can say /how/ accurate it was.
}

impl Score {
    pub fn to_string(self) -> String {
        match self {
            Score::Accurate => "Correct".into(),
            Score::Miss => "Incorrect".into(),
        }
    }
}

pub trait Kadeu {
    fn front(&self) -> &String;
    fn back(&self) -> Box<dyn Display>;
    fn score(&self, answer: String) -> game::Score;
}

pub mod game {

    pub enum Score {
        Accurate,
        Miss,
    }
}

pub enum KCard {
    Simple(String, String),
    List(String, Vec<String>),
}

impl KCard {
    pub fn make(self) -> Box<dyn Kadeu> {
        match self {
            KCard::Simple(front, back) => Box::new(cards::Card::<String>::new(front, back)),
            KCard::List(front, back) => Box::new(cards::ListCard::<String>::new(front, back)),
        }
    }
}

pub mod cards {
    use crate::Kadeu;
    use std::fmt::Display;

    pub trait MakeCard {
        type BACK;
        fn new(key: String, value: Self::BACK) -> Self;
    }

    pub struct ListCard<T> {
        key: String,
        value: Vec<T>,
    }
    impl<T: PartialEq> MakeCard for ListCard<T> {
        type BACK = Vec<T>;
        fn new(key: String, value: Vec<T>) -> ListCard<T> {
            ListCard { key, value }
        }
    }
    impl<T> Kadeu for ListCard<T> {
        fn front(&self) -> &String {
            &self.key
        }
        fn back(&self) -> Box<dyn Display> {
            Box::new("foobar".to_string())
        }

        fn score(&self, answer: String) -> crate::game::Score {
            todo!()
        }
    }

    pub struct Card<T> {
        key: String,
        value: T,
    }

    impl<T: PartialEq> MakeCard for Card<T> {
        type BACK = T;
        fn new(key: String, value: Self::BACK) -> Card<T> {
            Card { key, value }
        }
    }

    impl<T> Kadeu for Card<T> {
        fn front(&self) -> &String {
            &self.key
        }
        fn back(&self) -> Box<dyn Display> {
            Box::new("foobar".to_string())
        }

        fn score(&self, answer: String) -> crate::game::Score {
            todo!()
        }
    }
    //TODO reimpl tests here.
}
