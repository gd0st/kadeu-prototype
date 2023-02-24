use crate::{lib::Deck, read_and_strip};
pub mod schedule {
    use crate::lib::Card;
    use rand::{seq::SliceRandom, thread_rng};

    pub struct Random {}

    //can't figure out lifetimes yet
    impl<'a> Random {
        pub fn schedule(mut cards: Vec<&'a Card>) -> Vec<&'a Card> {
            cards.shuffle(&mut thread_rng());
            cards
        }
    }
}

pub trait Game {
    fn play(deck: &Deck) -> Vec<String>;
}

enum GameMode {
    Practice,
    Typed,
}

pub struct Practice;

impl Game for Practice {
    fn play(deck: &Deck) -> Vec<String> {
        //let cards = schedule::Random::schedule(deck.cards());
        for card in deck.cards() {
            println!("{}", card.challenge());
            let input = read_and_strip();
            for answer in card.answers() {
                println!("- {}", answer);
            }
        }
        vec![]
    }
}
pub struct Typed;
impl Game for Typed {
    fn play(deck: &Deck) -> Vec<String> {
        let mut log: Vec<String> = Vec::new();

        for card in deck.cards() {
            println!("{}", card.challenge());
            let input = read_and_strip();
            for answer in card.answers() {
                println!("- {}", answer);
            }
        }
        log
    }
}
