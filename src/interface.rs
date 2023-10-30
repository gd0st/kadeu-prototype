use crate::model::{Card, CardBack};
use controller::{Controller, Progress, Score, Strategy};
use std::fmt::Display;


pub trait Judge {
    fn validate(&self, answer: &String) -> Score;
}

pub trait KCard {
    fn prompt(&self) -> String;
    fn score(&self, answer: &String) -> Score;
}

impl<T, U> KCard for Card<T, U>
where T: Display,
    U: Judge,
{
    fn prompt(&self) -> String {
        self.front().to_string()
    }
    fn score(&self, answer: &String) -> Score {
        self.back().validate(answer)
    }
}

impl Judge for CardBack {
    fn validate(&self, answer: &String) -> Score {
        match self {
            CardBack::Word(target) => {
                if target == answer {
                    return Score::Hit;
                }
                Score::Miss
            }
        }
    }
}


pub mod controller {

    pub trait Strategy<T> {
        fn next<'a>(&self, progress: Vec<&'a Progress<T>>) -> Option<&'a T>;
        fn unanswered<'a>(&self, progress: Vec<&'a Progress<T>>) -> Vec<&'a Progress<T>> {
            progress.into_iter().filter(|progress| progress.score.is_none()).collect()
        }

        fn answered<'a>(&self, progress: Vec<&'a Progress<T>>) -> Vec<&'a Progress<T>> {
            progress.into_iter().filter(|progress| progress.score.is_some()).collect()
        }
    }

    pub enum Score {
        Hit,
        Partial(f64),
        Miss,
    }
    pub struct Progress<T> {
        item: T,
        score: Option<Score>
    }

    impl<T> Progress<T> {
        pub fn new(item: T, score: Option<Score>) -> Self {
            Self {
                item,
                score
            }
        }
        pub fn score(&self) -> Option<&Score> {
            if let Some(score) = &self.score {
                Some(score)
            } else {
                None
            }

        }

        pub fn item(&self) -> &T {
            &self.item
        }
    }
    pub struct Controller<T, U> {
        progress: Vec<Progress<T>>,
        strategy: U,
    }

    impl<T, U> Controller<T, U> {

        pub fn new(items: Vec<T>, strategy: U) -> Self {
            Self {
                progress: items.into_iter().map(|card| Progress::new(card, None)).collect(),
                strategy
            }
        }
    }

    impl<T, U: Strategy<T>> Controller<T, U> {
        pub fn next(&self) -> Option<&T> {
            self.strategy.next(self.progress.iter().collect())
        }
    }

}

impl<T, U> Controller<T, U>
where
    T: KCard,
    U: Strategy<T>
{

    fn input(&self, answer: &String) -> Score {
        todo!()
    }
}

pub struct Linear;
impl<T> Strategy<T> for Linear {

    fn next<'a>(&self, progress: Vec<&'a Progress<T>>) -> Option<&'a T> {
        let mut unanswered = self.unanswered(progress);
        if let Some(progress) = unanswered.pop() {
            Some(progress.card())
        } else {
            None
        }
    }
}

impl<KCard> Progress<KCard> {
    fn card(&self) -> &KCard {
        self.item()
    }
}
