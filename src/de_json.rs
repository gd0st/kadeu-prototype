use serde::{Deserialize, Serialize};
use serde_json;

use crate::{Kadeu, KadeuDeck};

#[derive(Deserialize, Serialize)]
struct Deck {
    title: String,
    cards: Vec<Card>,
}

impl KadeuDeck for Deck {
    fn title(&self) -> &String {
        &self.title
    }
    fn cards(&self) -> Vec<Box<dyn Kadeu>> {
        let mut vec: Vec<Box<dyn Kadeu>> = vec![];
        for card in &self.cards {
            let new_card: Box<dyn Kadeu> = Box::new(card.to_owned());
            vec.push(new_card);
        }
        vec
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct Card {
    front: String,
    back: serde_json::Value,
}

impl Kadeu for Card {
    fn front(&self) -> &String {
        &self.front
    }

    fn back(&self) -> String {
        match &self.back {
            serde_json::Value::Null => "".into(),
            serde_json::Value::Number(s) => s.to_string(),
            serde_json::Value::String(s) => s.to_string(),
            _ => "No Kadeu Support".into(),
        }
    }

    fn score(&self, answer: String) -> Result<bool, ()> {
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    #[test]
    fn from_any() {
        let data = r#"
{
  "title": "foo",
  "cards": [
    {
      "front": "foo",
      "back": ["bar"]
    },
    {
      "front": "foo",
      "back": "bar"
    }
  ]
}
"#;
        let _: Deck = serde_json::from_str(data).unwrap();
    }
}
