use std::io;
use tui::{backend::CrosstermBackend, Terminal};
mod ui;
enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), io::Error> {
    // TUI logic at this point after the flashcards have been loaded.
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to init Terminal GUI");
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
