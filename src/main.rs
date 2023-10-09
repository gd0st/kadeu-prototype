use clap::{arg, Command, Parser};
use kadeu::cards::back::CardBack;
use kadeu::config::{get_config, Settings};
use kadeu::de;
use kadeu::game::{self, KCard, KDeck};
use kadeu::load_deck;
use std::env;
use std::fs::{self, FileType};
use std::io;

const CONFIG_FILENAME: &str = "config.yml";

enum OperatingSystem {
    Unix,
    Windows,
}

fn cli() -> Command {
    Command::new("kadeu")
        .subcommand(Command::new("list").about("Get a list of flashcard sets available."))
        .subcommand(Command::new("play").about("Play a flashcard set"))
}

#[derive(Parser, Debug)]
struct Arg {
    #[arg(short, long)]
    flashcards: String,
    #[arg(long)]
    shuffle: bool,
}

enum Environment {
    Home,
}
use std::error::Error;

impl Environment {
    fn get(self) -> Result<String, env::VarError> {
        match self {
            Environment::Home => env::var("HOME"),
        }
    }
}

fn main() {
    let matches = cli().get_matches();

    let default_config_path = ".config/kadeu/config.yml";
    let kadeu_repo_root = ".kadeu";
    let home_path = Environment::Home.get().unwrap().to_string();
    println!("{}", &home_path);
    println!("{}", &default_config_path);
    let config = get_config(&format!("{}/{}", &home_path, default_config_path))
        .expect("Kadeu Settings file parsed.");
    let deck_name = "test.json";
    let filepath = format!("{}/{}/{}", home_path, config.repo, deck_name);
    println!("{}", &filepath);
    let deck = load_deck(filepath.as_str());
}
