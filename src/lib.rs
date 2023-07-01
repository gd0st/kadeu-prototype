use serde::{Deserialize, Serialize};
pub mod card;
use serde_json;

enum Score {
    Hit,
    Partial(f64),
    Miss,
}
trait DeckMaker<T> {
    fn make_deck(bytes: &str) -> Deck<T>;
}

#[derive(Deserialize, Serialize)]
enum KCard<T> {
    Simple(String, T),
}

#[derive(Deserialize, Serialize)]
struct Deck<T> {
    title: String,
    cards: Vec<KCard<T>>,
}

trait Kadeu {
    fn front(&self) -> &String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> Score;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple_card() {
        let data = r#"
{
  "title": "foo",
  "cards": [
  {
  "front": "foo",
  "back": "bar"
},
{
"front": "foo",
"back": ["bar"]
}
]

}
"#;
    }
}
