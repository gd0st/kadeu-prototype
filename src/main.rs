use clap::Parser;
use kadeu::{Card, Deck, SimpleCard};
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

    let card: SimpleCard = SimpleCard::new(
        "What group of organism is a fungus".to_string(),
        "eukaryotic".to_string(),
    );
    let next_card = SimpleCard::new(
        "What is Cytahus collocially known as?".to_string(),
        "Bird's nest fungi".to_string(),
    );

    let cards = vec![card, next_card];

    for card in cards {
        println!("!> {}", card.front());
        print!("?> ");
        let mut answer = String::new();
        io::stdout().flush().expect("stdout flush");
        read_to_buff(&mut answer);
        println!("> {}", card.back());
        // No reason to ask if yay or no if no benefit
        io::stdout().flush().expect("stdout flush");
        // just print the next question or wait for a continue?
        // might be useful to have when going into more advanced terminal interfaces.
        read_to_buff(&mut answer);

        // answer
        // feedback
        // continue
    }
    //let flashcards_filepath = args.flashcards;
    // TODO flashcard reader needs to be implemented after core game loop...
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
