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
