use crate::config::Config;
use objects::{Card, Deck};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
// A lot of this code was inspired by other tui examples
// So thank you to those that came before me.
//
pub struct App {
    pub should_quit: bool,
    decks: Vec<Deck>,
    pub curr_deck: Option<Deck>,
    pub config: Config,
    //pub terminal_width: u16,
    //pub terminal_height: u16,
}

trait Schedule {
    fn schedule(cards: Vec<&Card>) -> Vec<&Card>;
}

pub mod objects {

    use serde::Deserialize;

    #[derive(Deserialize, Clone, Debug)]
    pub struct Deck {
        pub title: String, // TODO deprecate
        pub tags: Vec<String>,
        pub cards: Vec<Card>,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Card {
        pub challenge: String,
        pub targets: Vec<String>,
    }
    impl Card {
        pub fn new(challenge: String, targets: Vec<String>) -> Card {
            Card { challenge, targets }
        }
    }
}

// Database for all of the decks..
//
enum DecksDBError {
    DeckLocationMoved(String),
}

pub trait DeckDB {
    // TODO This should be a Result instead of option?
    // TODO Might be better error handling, not sure yet.
    fn find_from_string(&self, string: String) -> Option<Deck>;
    fn tags(self) -> Vec<String>;
    fn titles(&self) -> Vec<&str>;
}

pub struct DeckFs {
    decks: Vec<Deck>,
}

impl DeckDB for DeckFs {
    fn find_from_string(&self, string: String) -> Option<Deck> {
        for deck in &self.decks {
            if deck.title == string {
                return Some(deck.clone());
            }
        }
        None
    }
    fn tags(self) -> Vec<String> {
        let mut tags: Vec<String> = Vec::new();
        for deck in self.decks {
            tags.append(&mut deck.tags.clone());
        }
        tags
    }

    fn titles(&self) -> Vec<&str> {
        let mut titles: Vec<&str> = Vec::new();

        for deck in &self.decks {
            titles.push(deck.title.as_str());
        }
        // TODO Figure out how to just map this shit...
        titles
    }
}

pub struct DeckFsConfig {
    pub root_directory: PathBuf,
}

use serde_json;
use std::fs;
use std::io;
impl DeckFs {
    pub fn new(config: DeckFsConfig) -> io::Result<DeckFs> {
        let entries = fs::read_dir(&config.root_directory)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let mut decks: Vec<Deck> = Vec::new();
        for entry in entries {
            let buff = fs::read_to_string(entry).unwrap();
            let de_deck: serde_json::Result<Deck> = serde_json::from_str(&buff);
            if let Err(e) = &de_deck {
                dbg!(e);
            }
            match de_deck {
                Ok(deck) => decks.push(deck),
                Err(_) => {}
            }
        }
        Ok(DeckFs { decks })
    }

    pub fn get_decks(&self) -> Vec<&Deck> {
        let mut decks: Vec<&Deck> = Vec::new();
        for deck in &self.decks {
            decks.push(deck);
        }

        decks
    }
}

//impl DecksDB<'a, 'b> for DecksFS<'a, 'b> {
//
//fn refresh(deck: ')
//
//}

//impl App {
//pub fn new(decks: Vec<Deck>, config: Config) -> App {
//App {
//should_quit: false,
//decks,
//curr_deck: None,
//config,
//}
//}
//pub fn iter_decks(&self) -> Vec<&Deck> {
//self.decks.iter().collect()
//}
//
//pub fn select_deck(&self, odeck: &Deck) {
//for deck in &self.decks {
//if (deck == odeck) {
//self.curr_deck
//}
//}
//}
//}
