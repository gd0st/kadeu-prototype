use clap::Parser;
use kadeu::game;
use kadeu::{de, de::DeckDeserializer, util, Card, Kadeu, KadeuDeck};

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
    let deck = de::Json::deserialize(filedata.as_str())
        .expect("Deck<String> resolved from json filedata.");
    let game = game::Game::new(deck.cards(), game::Mode::Practice);
    for card in game.cards() {
        let mut answer: String = String::new();
        println!("!>{}", card.front());
        print!("?>");
        let _ = io::stdout().flush();
        util::read_to_buff(&mut answer);
        let score = card.score(answer);
        if score {
            println!("Correct!");
        } else {
            println!("Incorrect");
        }
    }
}
