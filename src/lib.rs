use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod Cards {}

trait Validator {
    fn valid(&self, answer: &Answer) -> bool;
}

#[derive(Deserialize)]
struct Deck {
    title: String,
    tags: Option<Vec<String>>,
    description: Option<String>,
    cards: Vec<Card>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Tag {
    Single(String),
    Many(Vec<String>),
}

#[derive(Deserialize)]
struct Card {
    tags: Option<Tag>,
    challenge: String,
    target: Target,
}

impl Target {
    fn is_valid(&self, input: &str) -> bool {
        match self {
            Target::Simple(target) => input == target,
            Target::Multi(targets) => targets.iter().any(|target| *target == *input),
        }
    }
}

struct Answer(String);

impl Validator for Card {
    fn valid(&self, answer: &Answer) -> bool {
        self.target.is_valid(answer.0.as_str())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Target {
    Simple(String),
    Multi(Vec<String>),
}

#[cfg(test)]
mod tests {
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
}
