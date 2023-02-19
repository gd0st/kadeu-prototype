use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use serde::Deserialize;
use std::sync::mpsc;
use std::{
    io, thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Deserialize)]
struct Deck {
    title: String,
    tags: Option<Vec<String>>,
    description: Option<String>,
    cards: Vec<Card>,
}

#[derive(Deserialize)]
struct Card {
    tags: Option<Tag>,
    challenge: String,
    target: Target,
}
struct Answer(String);

#[derive(Deserialize)]
#[serde(untagged)]
enum Tag {
    Single(String),
    Many(Vec<String>),
}

impl Validator for Card {
    fn valid(&self, answer: &Answer) -> bool {
        self.target.is_valid(answer.0.as_str())
    }
}

//#[derive(Deserialize)]
//struct Card {
//challenge: String,
//target: ,
//}
#[derive(Deserialize)]
#[serde(untagged)]
enum Target {
    Simple(String),
    Multi(Vec<String>),
}
impl Target {
    fn is_valid(&self, input: &str) -> bool {
        match self {
            Target::Simple(target) => input == target,
            Target::Multi(targets) => targets.iter().any(|target| *target == *input),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum FrameMode {
    FlashCard,
    Select,
}

impl From<FrameMode> for usize {
    fn from(input: FrameMode) -> usize {
        match input {
            FrameMode::FlashCard => 0,
            FrameMode::Select => 1,
        }
    }
}

fn main() -> Result<(), io::Error> {
    // TUI logic at this point after the flashcards have been loaded.
    //
    //
    //
    //

    let mut title = "something";
    let mut content = "other";
    enable_raw_mode().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tick_rate = Duration::from_millis(200);
        let mut last_tick = Instant::now();

        loop {
            let mut timeout;
            match tick_rate.checked_sub(last_tick.elapsed()) {
                Some(time) => timeout = time,
                None => timeout = Duration::ZERO,
            }
            if event::poll(timeout).expect("Failed to poll for event.") {
                if let event::Event::Key(key) = event::read().expect("Failed to read event.") {
                    tx.send(Event::Input(key))
                        .expect("Faileed to send event into tx");
                }
                break;
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to init Terminal GUI");

    // loops through and checks the receive for new characters.
    // If q then it breaks the loop
    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            //let block = Block::default().title("Block").borders(Borders::ALL);
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10), // Deck Viewer
                        Constraint::Percentage(80), // Current Deck
                        Constraint::Percentage(10), // History and time
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let theme = Style::default().fg(Color::LightBlue).bg(Color::Magenta);
            frame.render_widget(
                ui::DeckViewer::new("foo".to_string(), theme.clone()),
                chunks[0],
            );
            frame.render_widget(
                ui::Game::new("bar".to_string(), "practice".to_string(), theme),
                chunks[1],
            );
        });
        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                event::KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}

impl Config {
    pub fn new(directory: String) -> Config {
        Config { directory }
    }
}

struct Config {
    directory: String,
}

struct FlashCard {
    question: String,
    targets: Targets,
}

// deserializer next?
//
//
struct Targets(Vec<String>);

impl FlashCard {
    pub fn new(question: String, targets: Vec<String>) -> FlashCard {
        FlashCard {
            question,
            targets: Targets(targets),
        }
    }
    pub fn from(question: String, targets: Targets) -> FlashCard {
        FlashCard { question, targets }
    }
}

impl Validator for FlashCard {
    fn valid(&self, answer: &Answer) -> bool {
        self.targets.0.iter().any(|target| *target == *answer.0)
    }
}

trait Validator {
    fn valid(&self, answer: &Answer) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FlashCard;

    #[test]
    fn make_flashcard() {
        let question = "foo".to_string();
        let targets = vec!["bar".to_string()];

        let card = FlashCard::new(question, targets);
        assert_eq!(card.question, "foo")
    }
    #[test]
    fn new_flashcard() {
        let question = "foo".to_string();
        let targets = vec!["bar".to_string()];

        let card = FlashCard::new(question, targets);
        let answer = Answer("bar".to_string());

        assert!(card.valid(&answer))
    }

    #[test]
    fn from_question_targets() {
        let targets = Targets(vec!["bar".to_string()]);
        let question = "foo".to_string();

        let card = FlashCard::from(question, targets);
        assert_eq!(card.question, "foo".to_string());
    }
}

mod ui {
    use std::io;
    use tui::backend::CrosstermBackend;
    use tui::buffer::Buffer;
    use tui::layout::{Constraint, Direction, Layout, Rect};
    use tui::style::{Color, Style};
    use tui::terminal::Frame;
    use tui::widgets::{Block, Borders, Widget};

    pub struct Settings {
        flashdir: String, // Location to look for flashcards.
    }

    pub struct Game {
        deck_title: String,
        mode: String,
        theme: Style,
    }

    impl Game {
        pub fn new(deck_title: String, mode: String, theme: Style) -> Self {
            Game {
                deck_title,
                mode,
                theme,
            }
        }
    }

    impl Widget for Game {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let mut block = Block::default().title(self.deck_title).style(self.theme);
            block = block.borders(Borders::ALL);
            Block::render(block, area, buf)
        }
    }

    pub struct DeckViewer {
        deck_titles: Vec<String>,
        directory: String,
        theme: Style,
    }
    impl DeckViewer {
        pub fn new(directory: String, theme: Style) -> Self {
            DeckViewer {
                deck_titles: Vec::new(),
                directory,
                theme,
            }
        }
    }

    impl Widget for DeckViewer {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let mut block = Block::default().title(self.directory).style(self.theme);
            Block::render(block, area, buf)
        }
    }

    trait IntoBlock {
        fn into_block(&self) -> Block;
    }

    pub fn layout(frame: Frame<CrosstermBackend<io::Stdout>>, settings: &Settings) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(frame.size())
    }
}
