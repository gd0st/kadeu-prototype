use clap::Parser;
use kadeu::{
    card::{Card, Deck},
    de,
    de::DeckDeserializer,
    util, Kadeu,
};

use std::io::{self, Write};
#[derive(Parser, Debug)]
struct Arg {
    #[arg(short, long)]
    flashcards: String,
}
fn main() {
    let args = Arg::parse();
    let filedata =
        util::read_filepath(args.flashcards).expect("Read file data from --flashcard path.");
    let deck = de::json::Json::deserialize(filedata.as_str())
        .expect("Deck<String> resolved from json filedata.");

    for card in deck.cards() {
        println!("!>{}", card.front());
        let mut answer: String = String::new();

        print!("?>");
        let _ = io::stdout().flush();
        util::read_to_buff(&mut answer);
        if card.score(answer) {
            println!("Correct!");
        } else {
            println!("Incorrect: {}", card.back());
        }
    }
}
