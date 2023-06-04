use serde;

pub struct Challenge {
    question: String,
    answer: String,
}

impl Challenge {
    pub fn new(question: String, answer: String) -> Challenge {
        Challenge { question, answer }
    }
}

pub trait ChallengeInterface {
    fn challenge(&self) -> Challenge;
}

impl ChallengeInterface for SimpleCard {
    fn challenge(&self) -> Challenge {
        Challenge::new(self.front.clone(), self.back.clone())
    }
}

pub enum Card {
    Simple(String, String),
    Complex(String, Vec<String>),
}

#[derive(PartialEq, serde::Deserialize)]
pub struct SimpleCard {
    front: String,
    back: String,
    tags: Vec<String>,
}

impl SimpleCard {
    fn new(front: String, back: String, tags: Vec<String>) -> SimpleCard {
        SimpleCard { front, back, tags }
    }
    fn push_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }
    fn is_valid(&self, answer: &String) -> bool {
        self.back == *answer
    }
    fn get_front(&self) -> &String {
        &self.front
    }
    fn get_back(&self) -> &String {
        &self.back
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleCard;
    #[test]
    fn make_simple_card() {
        let front: String = "Foo".into();
        let back: String = "Bar".into();
        let tags: Vec<String> = vec![];
        let card = SimpleCard::new(front, back, tags);
        assert_eq!(card.back, "Bar".to_string());
        assert_eq!(card.front, "Foo".to_string());
        assert!(card.tags.is_empty());
    }

    #[test]
    fn make_card_and_push_tag() {
        let front: String = "Foo".into();
        let back: String = "Bar".into();
        let tags: Vec<String> = vec![];
        let mut card = SimpleCard::new(front, back, tags);
        assert!(card.tags.is_empty());

        card.push_tag("Bar".into());
        assert_eq!(card.tags.len(), 1);
    }
}
