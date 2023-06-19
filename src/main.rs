use clap::Parser;
use kadeu::{Card, CardMaker, validate};
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
fn main() {
    let args = GameArgs::parse();

    let questions: Vec<(&str, &str)> = vec![
        ("What group of organism is a fungus", "eukaryotic")
    ];

    for question in questions {
        let card: Card<String> = Card::new(
            question.0.to_string(),
            question.1.to_string()
        );

        println!("!> {}", card.front());
        print!("?> ");
        io::stdout().flush().expect("stdout flush");
        let mut answer: String = String::new();
        read_to_buff(&mut answer);
        println!("Answer> {}", card.back());
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
