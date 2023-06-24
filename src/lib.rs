use std::{backtrace::Backtrace, fmt::Display};
pub mod de_csv;
pub mod kadeu;

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

impl Into<String> for Score {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Score {
    pub fn to_string(self) -> String {
        match self {
            Score::Accurate => "Hit".into(),
            Score::Miss => "Miss".into(),
        }
    }
}

pub trait Card {
    type BACK;
    fn new(key: String, value: Self::BACK) -> Self;
    fn key(&self) -> &String;
    fn value(&self) -> &Self::BACK;
}

pub struct CardList<T>(String, Vec<T>);
impl<T: PartialEq> Card for CardList<T> {
    type BACK = Vec<T>;
    fn new(key: String, value: Self::BACK) -> Self {
        CardList(key, value)
    }
    fn key(&self) -> &String {
        &self.0
    }
    fn value(&self) -> &Self::BACK {
        &self.1
    }
}

pub struct SimpleCard<T>(String, T);

impl<T: PartialEq> Card for SimpleCard<T> {
    type BACK = T;
    fn new(key: String, value: Self::BACK) -> Self {
        SimpleCard(key, value)
    }
    fn key(&self) -> &String {
        &self.0
    }
    fn value(&self) -> &Self::BACK {
        &self.1
    }
}

#[cfg(test)]
mod test {
    use crate::{Card, CardList, SimpleCard};

    #[test]
    fn make_simple_card() {
        let front = "foo";
        let back = "bar".to_string();
        let card = SimpleCard::new(front.to_string(), back);
        assert_eq!(front.to_string(), *card.key())
    }

    #[test]
    fn make_list_card() {
        let front = "foo";
        let back = vec![String::from("bar")];
        let card = CardList::new(front.to_string(), back);
        assert_eq!(front.to_string(), *card.key())
    }
}
