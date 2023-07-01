use crate::{Card, Deck, Kadeu};
use serde::{Deserialize, Serialize};
use serde_json;

impl Kadeu for Card<String, serde_json::Value> {
    fn front(&self) -> &String {
        &self.front
    }

    fn back(&self) -> String {
        self.back.to_string()
    }
    fn score(&self, answer: String) -> bool {
        self.value().to_string() == answer
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
        let _: Deck<serde_json::Value> = serde_json::from_str(data).unwrap();
    }
}
