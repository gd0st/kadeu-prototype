use crate::{Card, Score, SimpleCard};
pub trait Kadeu {
    fn front(&self) -> String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> Result<Score, String>;
}

impl Kadeu for SimpleCard<String> {
    fn front(&self) -> String {
        self.key().to_owned()
    }
    fn back(&self) -> String {
        self.value().to_owned()
    }
    fn score(&self, answer: String) -> Result<Score, String> {
        if self.back() == answer {
            Ok(Score::Accurate)
        } else {
            Ok(Score::Miss)
        }
    }
}

impl Kadeu for SimpleCard<usize> {
    fn front(&self) -> String {
        self.key().to_owned()
    }
    fn back(&self) -> String {
        self.value().to_string()
    }
    fn score(&self, answer: String) -> Result<Score, String> {
        let parsed_answer = answer.parse::<usize>();
        match parsed_answer {
            Ok(answer) => {
                if *self.value() == answer {
                    return Ok(Score::Accurate);
                } else {
                    return Ok(Score::Miss);
                }
            }
            Err(_) => return Err("Failed to parse answer for number.".to_string()),
        }
    }
}

pub enum KCard<T> {
    Card(String, T),
}

impl KCard<String> {
    pub fn make(self) -> Box<dyn Kadeu> {
        match self {
            KCard::Card(front, back) => Box::new(SimpleCard::new(front, back)),
        }
    }
}

impl KCard<usize> {
    pub fn make(self) -> Box<dyn Kadeu> {
        match self {
            KCard::Card(front, back) => Box::new(SimpleCard::new(front, back)),
        }
    }
}

#[cfg(test)]
mod test {
    //TODO
}
