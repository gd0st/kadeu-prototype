use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Compliance {
    Min(f32),
    Max(f32),
    Strict,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum CardBack<T> {
    Simple(T),
    Multi(Vec<T>),
}
#[derive(Deserialize, Serialize)]
struct Card<T> {
    front: String,
    back: CardBack<T>,
    compliance: Option<Compliance>,
}

#[derive(Deserialize, Serialize)]
struct Deck<T> {
    title: String,
    cards: Vec<Card<T>>,
}

trait Kadeu {
    fn front(&self) -> &String;
    fn back(&self) -> String;
    fn answer(&self, answer: String);
}

impl Kadeu for Card<String> {
    fn front(&self) -> &String {
        &self.front
    }
    fn back(&self) -> String {
        match &self.back {
            CardBack::Simple(s) => s.to_string(),
            _ => todo!(),
        }
    }
    fn answer(&self, answer: String) {}
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
      "back": ["bar"],
      "compliance": { "min": 32 }
    },
    {
      "front": "foo",
      "back": "bar"
    }
  ]
}
"#;
        let _: Deck<String> = serde_json::from_str(data).unwrap();
    }
}
