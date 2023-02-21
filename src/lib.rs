use serde::{Deserialize, Serialize};
use serde_json::Result;

trait Validator {
    fn valid(&self, answer: &Answer) -> bool;
}

#[derive(Deserialize)]
pub struct Deck {
    title: String,
    tags: Option<Vec<String>>,
    description: Option<String>,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(title: String, cards: Vec<Card>) -> Deck {
        Deck {
            title,
            cards,
            tags: None,
            description: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Tag {
    Single(String),
    Many(Vec<String>),
}

#[derive(Deserialize)]
pub struct Card {
    tags: Option<Tag>,
    challenge: String,
    target: Target,
}

impl Card {
    pub fn new(challenge: String, target: Target) -> Card {
        Card {
            challenge,
            target,
            tags: None,
        }
    }
}

impl Target {
    fn is_valid(&self, input: &str) -> bool {
        match self {
            Target::Simple(target) => input == target,
            Target::Multi(targets) => targets.iter().any(|target| *target == *input),
        }
    }
}

pub struct Answer(String);

impl Validator for Card {
    fn valid(&self, answer: &Answer) -> bool {
        self.target.is_valid(answer.0.as_str())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Target {
    Simple(String),
    Multi(Vec<String>),
}

mod game {
    use super::{Card, Deck};

    pub struct Practice {
        deck: Deck,
    }

    enum GameMode {
        Practice,
    }

    ///! docs
    ///!
    ///! The rules of the game follow a simple 8 bit pattern that
    ///! Will dictate what is allowed during the game.
    ///! So for instance 1101 is practice mode.
    ///! 0111 is a soft test
    ///! 1010 Would be a hard test
    struct GameRules(u8);
    pub struct Game {
        deck: Deck,
        mode: GameRules,
    }
}

#[cfg(test)]
mod tests {
    use super::game::Game;
    use super::game::Practice;
    use super::*;
    use serde_json;
    #[test]
    fn de_into_single_target_card() {
        let input = r#"
            {"challenge": "foobar", "target": "bizz", "tags": ["bazz"]}
        "#;
        let card: Card = serde_json::from_str(input).unwrap();
        let answer = Answer("bizz".to_string());
        assert!(card.valid(&answer))
    }

    #[test]
    fn de_into_multiple_target_card() {
        let input = r#"

{"challenge": "foo", "target": ["bar", "bazz"], "tags": ["bizz"]}

"#;
        let card: Card = serde_json::from_str(input).unwrap();

        let answer = Answer("bar".to_string());

        assert!(card.valid(&answer));
    }

    #[test]
    fn build_practice_game() {
        let cards = vec![Card::new(
            "foobar".to_string(),
            Target::Simple("bazz".to_string()),
        )];

        let deck = Deck::new("Simple deck for foo.".to_string(), cards);

        let practice_game = game::Practice::new(deck);
        assert!(true);
    }
}
