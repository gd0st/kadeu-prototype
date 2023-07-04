use crate::card::Deck;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
      "back": "bar"
    },
    {
        "front": {
            "display": "$1, Present",
            "options": ["Yo", "Tu", "Il/Elle/Usted", "Nosotros", "Ellos/Ellas/Ustedes"]
        },
        "back": {
            {
                "Yo": "Estoy",
                "Tu": "Estas",
                "Il/Elle/Usted": "Esta",
                "Nosotros": "Estamos",
                "Ellos/Ellas/Ustedes": "Estan"
            }
        }
    }
  ]
}
"#;
        let _: Deck<String> = serde_json::from_str(data).unwrap();
    }
}
