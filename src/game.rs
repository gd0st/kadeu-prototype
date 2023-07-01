use crate::cards::{Kadeu, Score, KCard};
use rand::thread_rng;
use rand::seq::SliceRandom;
trait Schedule {
	fn cards(self) -> Vec<Box<dyn Kadeu>>;
}

pub struct Deck {
	cards: Vec<Box<dyn Kadeu>>
}

impl Deck {
	pub fn new(cards: Vec<Box<dyn Kadeu>>) -> Self {
		Deck {
			cards
		}
	}
	fn shuffle(&mut self) {
		self.cards.shuffle(&mut thread_rng())
	}
	fn push(&mut self, card: Box<dyn Kadeu>) {
		self.cards.push(card)
	}
	fn pop(&mut self) -> Option<Box<dyn Kadeu>> {
		self.cards.pop()
	}
}

pub enum Message {
	Answer(String),
	Prompt(String),
	Score(String),
	Stop,
	Reset,
}

pub enum GameMode {
	Practice,
	Test,
	Hardcore
}
pub enum Queue {
	Random,
	Linear,
}

pub struct Game {
	deck: Deck,
	mode: GameMode,
	queue: Queue
}

impl Game {
	pub fn new(deck: Deck, mode: GameMode, queue: Queue) -> Self {
		Game {
			deck,
			mode,
			queue
		}
	}
	pub fn next(&self) -> Message {
		Message::Prompt("FOO".to_string())
	}
}

impl From<Vec<KCard>> for Deck {
	fn from(value: Vec<KCard>) -> Self {
		Deck::new(value.iter().map(|card| {
			card.make()
		}).collect())
	}

}

#[cfg(test)]
mod test {
    use crate::cards::KCard;
	use super::*;

	#[test]
	fn make_deck() {
		let cards = vec![
			KCard::Simple("foo".to_string(), "bar".to_string()),
			KCard::Simple("foo".to_string(), 42),
		];
		let deck = Deck::from(cards);
	}

}


