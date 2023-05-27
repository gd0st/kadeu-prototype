#[derive(PartialEq)]
struct SimpleCard {
    front: String,
    back: String,
    tags: Vec<String>,
}

impl Card for SimpleCard {
    fn new(front: String, back: String, tags: Vec<String>) -> SimpleCard {
        SimpleCard { front, back, tags }
    }
    fn push_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    fn flip(&self) -> SimpleCard {
        SimpleCard {
            front: self.back.clone(),
            back: self.front.clone(),
            tags: self.tags.clone(),
        }
    }
}

trait Card {
    fn new(front: String, back: String, tags: Vec<String>) -> Self;
    fn push_tag(&mut self, tag: String);
    fn flip(&self) -> Self;
}

impl Validator for SimpleCard {
    fn is_valid(&self, tag: &String) -> bool {
        self.back == *tag
    }
}

trait Validator {
    fn is_valid(&self, tag: &String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::SimpleCard;
    #[test]
    fn make_card() {
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
