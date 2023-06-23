use clap::Parser;
use kadeu::{KCard, Kadeu};
use std::fmt::Display;
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

enum Score {
    Accurate,
    Miss,
}

fn get_cards(filepath: String) -> Vec<KCard> {
    todo!()
}

// Compliancy here.

fn main() {
    let args = GameArgs::parse();

    let fcard = KCard::Simple(
        "Who is the 44th president.".to_string(),
        "Barack Obama".to_string(),
    );
    let scard = KCard::Simple(
        "What is capital of Germany".to_string(),
        "Berlin".to_string(),
    );
    let tcard = KCard::List(
        "Name a state in the DMV.".to_string(),
        vec![
            "Maryland".to_string(),
            "Virginia".to_string(),
            "DC".to_string(),
        ],
    );

    let kadeus: Vec<KCard> = vec![fcard, scard, tcard];

    for kadeu in kadeus {
        let card = kadeu.make();
        println!("{}", card.front());
        println!("{}", card.back());
        let mut answer = String::new();
        io::stdout().flush().expect("Stdout flushed");
        read_to_buff(&mut answer);
        let score = card.score(answer);
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
