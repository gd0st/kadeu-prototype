use clap::Parser;
use kadeu::{Card, CardMaker, CanDisplay};
use kadeu::game::{Judge, Score, Compliancy};
use kadeu::game;
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

enum KadeuCard {
    Simple(String, String),
    Multi(String, Vec<String>)
}


fn main() {
    let args = GameArgs::parse();


    let question = KadeuCard::Simple(
        "What group of organism is a fungus".to_string(),
        "eukaryotic".to_string()
    );
    let questions: Vec<(&str, &str)> = vec![
        ("What group of organism is a fungus", "eukaryotic")
    ];

    for question in questions {
        let card: Card<String> = Card::new(
            "How many people have served the President of the USA?".to_string(),
            "45".to_string()
        );

        println!("!> {}", card.front());
        print!("?> ");

        io::stdout().flush().expect("stdout flush");
        let mut answer: String = String::new();
        read_to_buff(&mut answer);

        let compliancy: Compliancy = Compliancy::Strict;
        // TODO ref?
        let score = card.score(answer.to_owned(), compliancy);

        // TODO this is saying incorrect when it should be correct... probably has to do with input
        println!("Answer> {} -- {}", card.back(), score.to_string());
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
