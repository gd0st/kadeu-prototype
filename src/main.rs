mod core;
use crate::core::cards;
use clap::Parser;
use csv;
use std::io;

#[derive(Debug, Parser)]
#[command(author, version)]
struct GameArgs {
    // TODO allow for flashcard csv file.
    #[arg(long)]
    flashcards: String,
    #[arg(long)]
    shuffle: bool,
}
fn main() {
    let args = GameArgs::parse();

    let flashcards_filepath = args.flashcards;
    println!("{}", &flashcards_filepath);

    // TODO flashcard reader needs to be implemented after core game loop...
}

enum Schedule {
    Linear,
    Random,
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
