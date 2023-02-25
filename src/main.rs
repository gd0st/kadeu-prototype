mod lib;
use lib::{Card, Deck};
mod app;
use app::{DeckDB, DeckFs, DeckFsConfig};
mod game;

use game::schedule;
use game::{Game, Practice, Typed};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json;
mod ui;
use std::fs;
use std::io;

enum Event<I> {
    Input(I),
    Tick,
}

enum GameMode {
    Practice,
    Typed,
}

pub mod config {
    use home;
    use std::path::PathBuf;

    mod errors {
        #[derive(Debug)]
        pub enum ConfigError {
            NO_HOME(String),
        }
    }

    #[derive(Debug, Clone)]
    pub struct Config {
        pub root_directory: PathBuf,
    }

    pub fn get_config() -> Result<Config, errors::ConfigError> {
        if let Some(mut path) = home::home_dir() {
            path.push(".kadeu");
            Ok(Config {
                root_directory: path.into(),
            })
        } else {
            Err(errors::ConfigError::NO_HOME(
                "Could not find $HOME for the user agent".to_string(),
            ))
        }
    }
}

impl From<config::Config> for DeckFsConfig {
    fn from(config: config::Config) -> DeckFsConfig {
        let config_directory = config.root_directory.clone().push(".config");
        DeckFsConfig {
            root_directory: config.root_directory,
        }
    }
}

fn main() -> io::Result<()> {
    // TUI logic at this point after the flashcards have been loaded.
    //
    // Define a simple game flow
    //

    let config: config::Config = config::get_config().unwrap();

    let fs_config: DeckFsConfig = config.into();
    let db: DeckFs = DeckFs::new(fs_config)?;

    for title in db.titles() {
        println!("{:?}", title);
    }
    for title in db.titles() {
        println!("{:?}", title);
    }

    Ok(())

    //let valid_deck_paths = DeckPaths::new(&config).unwrap();
    //let decks: Vec<Deck> = valid_deck_paths
    //.paths
    //.iter()
    //.map(|x| Deck::from(x))
    //.collect();
    //let mut deck: Option<&Deck> = None;
    //
    //while deck.is_none() {
    //for path in decks.iter() {
    //println!("{}", path.title());
    //}
    //println!("Please Choose A Deck:");
    //let input = read_and_strip();
    //let mut temp: Vec<&Deck> = decks.iter().filter(|x| *x.title() == input).collect();
    //deck = temp.pop();
    //}
    //
    //
    //if let Some(deck) = deck {
    //println!("Found a deck! {}", deck.title())
    //}

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
    //println!("{}", answer)
    //}
    //}
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
