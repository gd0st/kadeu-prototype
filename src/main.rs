mod lib;
use lib::Deck;
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
mod ui;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() {
    // TUI logic at this point after the flashcards have been loaded.
    //
    // Define a simple game flow
    //
    let kadeu_directory = ".kadeu";
    let mut user_path = env::var_os("HOME").expect("Failed to source $HOME env path for the user.");
    user_path.push(format!("/{}", kadeu_directory));

    let file_meta = fs::metadata(&user_path).expect("Could not get metadata for user path");

    if file_meta.is_dir() {
        println!("Found a directory!")
    } else {
        println!("Not a directory!")
    }

    let directory =
        fs::read_dir(&user_path).expect("Could not list dir entry for Kadeu directory.");

    let mut decks: Vec<Deck> = Vec::new();
    for entry in directory {
        let unwrapped = entry.unwrap().path();
        if let Ok(mut file) = File::open(&unwrapped) {
            let mut buff = String::new();
            file.read_to_string(&mut buff).unwrap();
            let deck: serde_json::Result<Deck> = serde_json::from_str(&buff);
            match deck {
                Ok(deck) => {
                    dbg!(decks.push(deck));
                }
                Err(_) => eprintln!("{:?} Invalid deck.", &unwrapped),
            }
        }

        // Check the files if they are valid during the output.
    }

    // Load the deck
    // Hand the deck to Scheduler
    //
    // Scheduler returns the cards.
    //
}

mod schedule {
    use flashcards::{Card, Deck};
    trait Schedule {
        fn new(deck: Deck) -> Self;
        fn sequence(&self) -> Vec<&Card>;
    }
}

impl Config {
    pub fn new(directory: String) -> Config {
        Config { directory }
    }
}

struct Config {
    directory: String,
}
