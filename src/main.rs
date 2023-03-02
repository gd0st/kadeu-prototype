mod lib;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use lib::{Card, Deck};
mod app;
use app::{DeckDB, DeckFs, DeckFsConfig};
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
    text::{Span, Spans},
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

fn main() -> io::Result<()> {
    // What is the beginning of a user interface?
    // As in what is the bare minimum interface for a program to be built upon?
    // I think I need t accomplish making the program usable through command line arguments.
    // Afterwards the game needs to have a comprehensive interactive interface.
    // The interface will start with just stdio and the advance to the TUI
    // After the tui has been accomplished and features seem good enough,
    // The web browser and other parts can be accomplished

    let config: config::Config = config::get_config().unwrap();

    let fs_config: DeckFsConfig = config.into();
    let db: DeckFs = DeckFs::new(fs_config)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

    for title in db.titles() {
        println!("{}", title);
    }
    println!("Select a deck:");

    let mut input = String::new();

    loop {
        terminal.draw(|frame| {
            let mut items: Vec<ListItem> = Vec::new();

            for title in db.titles() {
                items.push(ListItem::new(title))
            }

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());
            let paragraph_block = Block::default()
                .title("Choose A Deck")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let text = vec![Spans::from(Span::raw(input.as_str()))];

            let paragraph = Paragraph::new(text).block(paragraph_block);
            //let decks_list = List::new(items).block(paragraph_block);
            frame.render_widget(paragraph, chunks[0])
        });

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => return Ok(()),
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    return Ok(());
                }
                _ => {}
            }
        } else {
            disable_raw_mode()?;
            return Ok(());
        }
    }
    return Ok(());
    let mut input = read_and_strip();

    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        let paragraph_block = Block::default()
            .title("Welcome to the World!")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));
        let paragraph = Paragraph::new("Hello World!").block(paragraph_block);
        frame.render_widget(paragraph, chunks[0])
    });

    disable_raw_mode()?;

    //Need randomizer
    // Game mode would have to be selected somewhere here.
    if let Some(game_deck) = db.find_from_string(input) {
        // Schedule would need to added somewhere in here for randommization.

        let mut cards = game_deck.cards;
        println!("{:?}", &cards[0]);
        cards.shuffle(&mut thread_rng());
        println!("{:?}", &cards[0]);
        for card in cards {
            println!("{}:", card.challenge);
            input = read_and_strip();
            if card.targets.iter().any(|x| x == &input) {
                println!("Yes!")
            } else {
                println!("No :(");
                println!("Acceptable Answers");
                card.targets.iter().for_each(|x| println!("{}", x));
            }
        }
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
