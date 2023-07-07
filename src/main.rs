use clap::Parser;
use kadeu::game;
use kadeu::{util, Card, Deck, Kadeu};

use std::io::{self, Write};
#[derive(Parser, Debug)]
struct Arg {
    #[arg(short, long)]
    flashcards: String,
    #[arg(long)]
    shuffle: bool,
}
fn main() {
    let args = Arg::parse();
    let filedata =
        util::read_filepath(args.flashcards).expect("Read file data from --flashcard path.");
    let mut deck: Deck<Card<String, String>> = serde_json::from_str(&filedata).unwrap();
    if args.shuffle {
        deck.shuffle();
    }
    let cards = deck.cards();
    for card in cards {
        let mut answer: String = String::new();
        println!("!>{}", card.front());
        print!("$>");
        let _ = io::stdout().flush();
        util::read_to_buff(&mut answer);
        let score = card.score(answer);
        if score {
            println!("OK!");
        } else {
            println!("!!! {}", card.back());
        }
    }
}
