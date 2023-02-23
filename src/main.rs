mod lib;
use lib::{Card, Deck};
use serde_json;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
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
    let mut home_path = env::var_os("HOME").expect("Failed to source $HOME env path for the user.");
    home_path.push(format!("/{}", kadeu_directory));

    let kadeu_path: PathBuf = home_path.into();
    let config: Config = kadeu_path.try_into().unwrap();
    let valid_deck_paths = DeckPaths::new(&config).unwrap();
    let decks: Vec<Deck> = valid_deck_paths
        .paths
        .iter()
        .map(|x| Deck::from(x))
        .collect();

    let mut deck: Option<&Deck> = None;

    while deck.is_none() {
        for path in decks.iter() {
            println!("{}", path.title());
        }
        println!("Please Choose A Deck:");
        let input = read_and_strip();
        let mut temp: Vec<&Deck> = decks.iter().filter(|x| *x.title() == input).collect();
        deck = temp.pop();
    }

    if let Some(deck) = deck {
        println!("Found a deck! {}", deck.title())
    }

    //let file_meta = fs::metadata(&user_path).expect("Could not get metadata for user path");
    //
    //if file_meta.is_dir() {
    //println!("Found a directory!")
    //} else {
    //println!("Not a directory!")
    //}
    //
    //let directory =
    //fs::read_dir(&user_path).expect("Could not list dir entry for Kadeu directory.");
    //
    //let mut decks: Vec<Deck> = Vec::new();
    //for entry in directory {
    //let unwrapped = entry.unwrap().path();
    //if let Ok(mut file) = File::open(&unwrapped) {
    //let mut buff = String::new();
    //file.read_to_string(&mut buff).unwrap();
    //let deck: serde_json::Result<Deck> = serde_json::from_str(&buff);
    //match deck {
    //Ok(deck) => {
    //decks.push(deck);
    //}
    //Err(_) => eprintln!("{:?} Invalid deck.", &unwrapped),
    //}
    //}
    //// Check the files if they are valid during the output.
    //}
    //
    ////better
    //decks.iter().for_each(|x| println!("{}", x.title()));
    //println!("Please Select:");
    //let input = read_strip()
    //
    //let mut possible_decks: Vec<&Deck> = decks.iter().filter(|x| x.title().eq(&input)).collect();
    //
    //if let Some(deck) = possible_decks.pop() {
    //// let game = init_game(deck, 0);
    //println!("Game is meant to start here.");
    //use schedule::Schedule;
    //let sequence = schedule::Random::schedule(deck.cards());
    //for card in sequence {
    //println!("Question: ");
    //let answer = read_strip();
    //println!("{}", answer)
    //}
    //}
}

enum DeckError {
    BadFormat(String),
}

impl From<&fs::DirEntry> for Deck {
    fn from(entry: &fs::DirEntry) -> Deck {
        serde_json::from_str(&fs::read_to_string(entry.path()).unwrap()).unwrap()
    }
}

struct DeckPaths {
    pub paths: Vec<DirEntry>,
    root_directory: String,
}

#[derive(Debug)]
enum DeckPathError {
    NoDirectory(String),
    CannotReadRootDirectory(String),
}
impl DeckPaths {
    fn new(config: &Config) -> Result<DeckPaths, DeckPathError> {
        let kadeu_root = config.root_directory.clone();
        let entries = match fs::read_dir(&kadeu_root) {
            Ok(files) => files,
            Err(err) => {
                return Err(DeckPathError::NoDirectory(err.to_string()));
            }
        };
        let mut valid_paths: Vec<DirEntry> = Vec::new();
        for entry in entries {
            if let Ok(file) = entry {
                let buff = fs::read_to_string(file.path()).unwrap();
                // Not really sure how to just match the serde_json part.
                let deck: serde_json::Result<Deck> = serde_json::from_str(&buff);
                if deck.is_ok() {
                    valid_paths.push(file)
                }
            }
        }
        Ok(DeckPaths {
            paths: valid_paths,
            root_directory: kadeu_root,
        })
    }
}

fn read_and_strip() -> String {
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

mod schedule {
    use crate::lib::{Card, Deck};
    use rand::{seq::SliceRandom, thread_rng};
    pub trait Schedule {
        fn schedule(cards: Vec<&Card>) -> Vec<&Card>;
    }

    pub struct Random {}

    impl Schedule for Random {
        fn schedule(mut cards: Vec<&Card>) -> Vec<&Card> {
            cards.shuffle(&mut thread_rng());
            cards
        }
    }

    // impl Schedule for Random {}
}

struct Config {
    pub root_directory: String,
}

impl From<PathBuf> for Config {
    fn from(path: PathBuf) -> Config {
        Config {
            root_directory: path.to_str().unwrap().to_string(),
        }
    }
}
