use clap::Parser;
use kadeu::{Card, Score, SimpleCard};
use std::io::{self, Write};

#[derive(Debug, Parser)]
#[command(author, version)]
struct GameArgs {
    // TODO allow for flashcard csv file.
    //#[arg(long)]
    //flashcards: String,
    //#[arg(long)]
    //shuffle: bool,
}

trait Kadeu {
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
    fn make(self) -> Box<dyn Kadeu> {
        match self {
            KCard::Card(front, back) => Box::new(SimpleCard::new(front, back)),
        }
    }
}

impl KCard<usize> {
    fn make(self) -> Box<dyn Kadeu> {
        match self {
            KCard::Card(front, back) => Box::new(SimpleCard::new(front, back)),
        }
    }
}

fn main() {
    let question = KCard::Card("Who is the 44th President".to_string(), "Obama".to_string());
    let squestion = KCard::Card("How many states are in the USA".to_string(), 50);

    let cards: Vec<Box<dyn Kadeu>> = vec![question.make(), squestion.make()];

    for card in cards {
        let mut answer = String::new();

        println!("!> {}", card.front());
        print!("?: ");
        io::stdout().flush().expect("Flushed stdout");
        read_to_buff(&mut answer);
        let score = card.score(answer).expect("Score response from Kadeu");
        println!("{}", score.to_string())
    }
}

fn read_to_buff(buff: &mut String) {
    let _ = io::stdin()
        .read_line(buff)
        .expect("Attempting stdio input.");

    if let Some('\n') = buff.chars().next_back() {
        buff.pop();
    }
    if let Some('\r') = buff.chars().next_back() {
        buff.pop();
    }
}

#[cfg(test)]
mod test {
    use kadeu::Score;

    use super::{KCard, Kadeu};

    #[test]
    fn make_string_kadeu() {
        let front = "What is the deepest man made hole?".to_string();
        let back = "Kola Superdeep Borehold".to_string();
        let card: Box<dyn Kadeu> = KCard::Card(front, back.to_owned()).make();
        let score = card.score(back.to_owned()).unwrap();
        assert_eq!(back.to_owned(), card.back());
        assert_eq!(score.to_string(), "Hit".to_string());
    }

    #[test]
    fn make_usize_kadeu() {
        let front = "How many feet deep is the Kola Superdeep Borehole?".to_string();
        let back = 40226;
        let card: Box<dyn Kadeu> = KCard::Card(front, back).make();
        let score = card.score(back.to_string()).unwrap();
        assert_eq!(back.to_string(), card.back());
        assert_eq!(score.to_string(), "Hit".to_string());
    }

    #[test]
    fn mixed_kadeu() {
        let cards = vec![
            KCard::Card(
                "Who is the third highest paid federal employee".to_string(),
                "The Postmaster General".to_string(),
            )
            .make(),
            KCard::Card(
                "How many K is the starting salary for the Postmaster General".to_string(),
                275,
            )
            .make(),
        ];

        cards.iter().for_each(|card| {
            // TODO need to engineer a better test.
            println!("{}", card.front());
            println!("{}", card.back());
        });
    }
}
