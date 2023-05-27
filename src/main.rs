mod lib;
use clap::Parser;
use lib::{Card, SimpleCard};
use std::io;

#[derive(Debug, Parser)]
#[command(author, version)]
struct GameArgs {
    // TODO allow for flashcard csv file.
    #[arg(long)]
    from_flashcards: String,
    #[arg(long)]
    shuffle: bool,
}
fn main() {
    let args = GameArgs::parse();
    println!("{}", args.shuffle);

    // TODO Start game engine
    // TODO
}

//TODO testing
trait Game<T: Card> {
    fn new(cards: Vec<T>) -> Self;
    fn is_valid_string(card: &T, answer: &String) -> bool;
}

struct SimpleGame<T: Card> {
    cards: Vec<T>,
}

impl<T: Card> Game<T> for SimpleGame<T> {
    fn new(cards: Vec<T>) -> SimpleGame<T> {
        SimpleGame { cards }
    }

    fn is_valid_string(card: &T, answer: &String) -> bool {
        card.is_valid(answer)
    }
}

fn read_to_buff(buff: &mut String) {
    let _ = io::stdin()
        .read_line(buff)
        .expect("Attempting stdio input.");
    println!("here");

    if let Some('\n') = buff.chars().next_back() {
        buff.pop();
    }
    if let Some('\r') = buff.chars().next_back() {
        buff.pop();
    }
}
