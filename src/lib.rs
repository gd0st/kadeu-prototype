use std::fmt::Display;



pub mod game {
    use super::Card;
    pub enum Compliancy {
        Strict,
        Minimum(f64),
        Maximum(f64)
    }
    impl Compliancy {
        fn passed(self, accuracy: f64) -> bool {
            match self {
                Compliancy::Strict => accuracy == 100 as f64,
                Compliancy::Maximum(max) => accuracy < max,
                Compliancy::Minimum(min) => accuracy > min
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



    pub trait Judge {
        fn score(&self, answer: String, compliancy: Compliancy) -> Score;
    }

    impl Judge for Card<String> {
        fn score(&self, answer: String, _: Compliancy) -> Score {
            if self.value == answer {
                Score::Accurate
            } else {
                Score::Miss
            }
        }
    }

    impl Judge for Card<Vec<String>> {
        fn score(&self, answer: String, compliancy: Compliancy) -> Score {
            let total = self.value.len();
            let mut score = 0;
            self.value.iter().for_each(|value| if *value == answer {
                score += 1;
            });

            let accuracy = (score / total) as f64;
            if compliancy.passed(accuracy) {
                return Score::Accurate
            }
            Score::Miss
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
