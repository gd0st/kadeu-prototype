use std::collections::VecDeque;
pub trait Judge {
    fn validate(&self, answer: &String) -> Option<Score>;
}

pub trait KCard {
    fn prompt(&self) -> String;
    fn score(&self, answer: &String) -> Option<Score>;
}

pub trait Scheduler<T> {
    fn sequence(&self, progress: &mut Vec<Progress<T>>);
}
pub enum Score {
    Hit,
    Partial(f64),
    Miss,
}
pub struct Progress<T> {
    item: T,
    score: Option<Score>,
}

impl<T> Progress<T> {
    fn new(item: T, score: Option<Score>) -> Self {
        Progress { item, score }
    }
}

pub struct Controller<T, U> {
    sequence: Vec<Progress<T>>,
    scheduler: U,
}

impl<T, U> Controller<T, U>
where
    U: Scheduler<T>,
{
    pub fn new(items: Vec<T>, scheduler: U) -> Self {
        let mut sequence = items
            .into_iter()
            .map(|item| Progress::new(item, None))
            .collect();
        scheduler.sequence(&mut sequence);
        Controller {
            sequence,
            scheduler,
        }
    }
}

impl<T: KCard, U: Scheduler<T>> Controller<T, U> {
    pub fn input(&mut self, answer: &String) {
        self.sequence.reverse();
        if let Some(progress) = self.sequence.pop() {
            let card = progress.item;
            let score = card.score(answer);
            self.sequence.reverse();
            self.sequence.push(Progress::new(card, score));
        }
        self.scheduler.sequence(&mut self.sequence);
    }

    pub fn next(&mut self) -> Option<&T> {
        if let Some(progress) = self.sequence.last() {
            Some(&progress.item)
        } else {
            None
        }
    }
}
