use clap::{arg, Command, Parser};
use kadeu::interface::controller;
use kadeu::interface::{Linear, controller::Controller};
use std::env;
use std::fs::{self, FileType};
use std::io;
use kadeu::model::{Card, Deck, CardBack};
use kadeu::de;

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
    let strategy = Linear;
    let controller = Controller::new(deck.cards(), strategy);
    println!("{:?}", controller.next());

    //let home_path = Environment::Home.get().unwrap().to_string();
    //let default_config_path = ".config/kadeu/config.yml";
    //let config = get_config(&format!("{}/{}", &home_path, default_config_path))
    //.expect("Kadeu Settings file parsed.");
    //
    //let matches = cli().get_matches();
    //
    //let repo_path = format!("{}/{}", &home_path, config.repo);
    //let repo = KadeuRepo::new(repo_path, RepoSource::Local);
    //repo.list();
}
