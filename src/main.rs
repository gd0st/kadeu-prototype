use clap::{arg, Command, Parser};
use kadeu::de;
use kadeu::model::{Card, CardBack, Deck};
use kadeu::{App, Linear, Message};
use serde_yaml::Sequence;
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
    from: String,
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
    let home_path = Environment::Home.get().unwrap().to_string();
    let args = Arg::parse();
    let text = fs::read_to_string(args.from).unwrap();
    let deck: Deck<String, CardBack> = de::Parser::Json.parse(text.as_str()).unwrap();
    let mut app = App::new(deck.cards(), Linear);
    let mut count = 0;
    while let Some(message) = app.receive() {
        match message {
            Message::Msg(text) => println!("{}", text),
        }

        app.send(Message::Msg("foobar".to_string()));
        count += 1;
        if count > 3 {
            break;
        }
    }
}
