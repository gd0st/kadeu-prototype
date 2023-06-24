use clap::Parser;
use kadeu::{Card, SimpleCard};
use std::io::{self, Write};

// Need simple app that can tell stdout what to render next.
// And receives input, need to read more code before I do that.

#[derive(Debug, Parser)]
#[command(author, version)]
struct GameArgs {
    // TODO allow for flashcard csv file.
    //#[arg(long)]
    //flashcards: String,
    //#[arg(long)]
    //shuffle: bool,
}

// Compliancy here.

trait Kadeu {
    fn front(&self) -> String;
    fn back(&self) -> String;
}

impl Kadeu for SimpleCard<String> {
    fn front(&self) -> String {
        self.key().to_owned()
    }
    fn back(&self) -> String {
        self.value().to_owned()
    }
}

pub enum KCard<T> {
    Card(String, T),
    CardList(String, Vec<T>),
}

fn main() {
    let question = KCard::Card("foo".to_string(), "bar".to_string());
    if let KCard::Card(front, back) = question {
        let card: SimpleCard<String> = SimpleCard::new(front, back);
        let score: String = card.score("bar".to_string()).into();
        println!("{}", card.front());
        println!("{}", card.back());
        println!("{}", score);
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
