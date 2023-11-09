use model::Deck;

pub mod config;
pub mod de;
pub mod errors;
pub mod game;
pub mod model;
use crate::game::{Controller, Judge, KCard, Scheduler, Score};
use crate::model::{Card, CardBack};
use std::cmp::Ordering;
use std::fmt::Display;
pub enum RepoSource {
    // TODO Will require network and async etc.
    Net,
    Local,
}

impl RepoSource {
    fn net() -> Self {
        Self::Net
    }

    fn local() -> Self {
        Self::Local
    }
}

pub struct App<T, U> {
    controller: Controller<T, U>,
}

impl<T, U> App<T, U>
where
    T: KCard,
    U: Scheduler<T>,
{
    pub fn new(items: Vec<T>, scheduler: U) -> Self {
        let controller: Controller<T, U> = Controller::new(items, scheduler);
        App { controller }
    }
    pub fn send(&mut self, message: Message) {
        match message {
            Message::Msg(text) => self.controller.input(&text),
        }
    }

    pub fn receive(&mut self) -> Option<Message> {
        if let Some(card) = self.controller.next() {
            Some(Message::Msg(card.prompt()))
        } else {
            None
        }
    }
}

pub struct Linear;
impl<T> Scheduler<T> for Linear {
    fn sequence(&self, progress: &mut Vec<game::Progress<T>>) {
        progress.reverse()
    }
}

pub enum Message {
    Msg(String),
}

pub struct KadeuRepo {
    source: RepoSource,
    path: String,
}
impl<T, U> KCard for Card<T, U>
where
    T: Display,
    U: Judge,
{
    fn prompt(&self) -> String {
        self.front().to_string()
    }
    fn score(&self, answer: &String) -> Option<Score> {
        self.back().validate(answer)
    }
}

impl<T, U> KCard for &Card<T, U>
where
    T: Display,
    U: Judge,
{
    fn prompt(&self) -> String {
        self.front().to_string()
    }
    fn score(&self, answer: &String) -> Option<Score> {
        self.back().validate(answer)
    }
}

impl Judge for CardBack {
    fn validate(&self, answer: &String) -> Option<Score> {
        match self {
            CardBack::Word(target) => match target.cmp(answer) {
                Ordering::Equal => Some(Score::Hit),
                _ => Some(Score::Miss),
            },
        }
    }
}
