mod app;
mod lib;
use app::objects;
use app::{DeckDB, DeckFs, DeckFsConfig};
use crossbeam::channel::SelectedOperation;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use lib::{Card, Deck};
mod game;
use game::schedule;
use game::{Game, Practice, Typed};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json;
use tui::widgets::canvas::Rectangle;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Terminal,
};
mod ui;
use std::fs;
use std::io;

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
        // Might be useful some other day.
        let _ = config.root_directory.clone().push(".config");
        DeckFsConfig {
            root_directory: config.root_directory,
        }
    }
}

// not really sure what to call this
struct Indexer {
    deck_list: Vec<String>,
    db: DeckFs,
}

fn new_db() -> io::Result<DeckFs> {
    let config: config::Config = config::get_config().unwrap();

    let fs_config: DeckFsConfig = config.into();
    DeckFs::new(fs_config)
}

fn main() -> io::Result<()> {
    let db = new_db()?;

    let decks = db.get_decks();

    println!("Available Decks:");
    let mut count: usize = 0;
    for deck in &decks {
        count += 1;
        println!("{}: {}", count, deck.title);
    }
    let mut deck_index: usize = 0;

    while deck_index < 1 || deck_index > count + 1 {
        println!("Select [1 - {}]", count);
        deck_index = read_and_strip()?.parse().unwrap_or(0)
    }

    let game_deck = decks[deck_index - 1];
    let mut cards = game_deck.cards.clone();
    cards.shuffle(&mut thread_rng());
    for card in &cards {
        println!("{}:", card.challenge);
        let input = read_and_strip()?;

        if card.targets.iter().any(|target| target == &input) {
            println!("Correct")
        } else {
            println!("Incorrect");
            println!("Answers:");
            card.targets
                .iter()
                .for_each(|target| println!("{}", target));
        }
    }

    Ok(())
}

fn read_and_strip() -> io::Result<String> {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff)?;

    if let Some('\n') = buff.chars().next_back() {
        buff.pop();
    }
    if let Some('\r') = buff.chars().next_back() {
        buff.pop();
    }

    Ok(buff)
}
