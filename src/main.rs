mod lib;
use lib::{Deck, Card};
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
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
                    decks.push(deck);
                }
                Err(_) => eprintln!("{:?} Invalid deck.", &unwrapped),
            }
        }
        // Check the files if they are valid during the output.
    }

    //better
    decks.iter().for_each(|x| println!("{}", x.title()));
    println!("Please Select:");
	let input = read_strip();

    let mut possible_decks: Vec<&Deck> = decks
        .iter()
        .filter(|x| x.title().eq(&input))
        .collect();

    if let Some(deck) = possible_decks.pop() {
        // let game = init_game(deck, 0);
		println!("Game is meant to start here.");
		use schedule::Schedule;
		let sequence = schedule::Random::schedule(deck.cards());
		for card in sequence {
			println!("Question: ");
			let answer = read_strip();
			println!("{}", answer)
		}
    }
}


fn read_strip() -> String {
    let mut buff_select = String::new();
    io::stdin().read_line(&mut buff_select).unwrap();

    if let Some('\n') = buff_select.chars().next_back() {
        buff_select.pop();
    }
    if let Some('\r') = buff_select.chars().next_back() {
        buff_select.pop();
    }

	buff_select
} 

// struct Game {
//     mode: u8,
//     deck: &Deck,
// }

// fn init_game(deck: &Deck, mode: u8) -> Game {
//     Game { mode, deck }
// }

mod schedule {
    use crate::lib::{Card, Deck};
	use rand::{thread_rng, seq::SliceRandom};
    pub trait Schedule {
		fn schedule(cards: Vec<&Card>) -> Vec<&Card>;
    }

	pub struct Random{}

	impl Schedule for Random {
		fn schedule(mut cards: Vec<&Card>) -> Vec<&Card> {
			cards.shuffle(&mut thread_rng());
			cards
		}
	}

    // impl Schedule for Random {}
}

impl Config {
    pub fn new(directory: String) -> Config {
        Config { directory }
    }
}

struct Config {
    directory: String,
}
