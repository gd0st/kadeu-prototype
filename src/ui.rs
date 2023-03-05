use crate::app;
use app::objects::{Card, Deck};
use std::io;
use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};

use tui::terminal::Frame;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph, Widget};

use tui::widgets::{List, ListItem};

fn format_deck_titles(decks: &Vec<Deck>) -> List {
    let list_items: Vec<ListItem> = Vec::new();

    for title in decks {}

    List::new(list_items)
}

pub mod text {

    pub mod deck {
        use crate::app::objects::Deck;
        pub mod text {
            use super::Deck;

            use tui::style::Style;
            use tui::text::Text;
            pub fn title(deck: &Deck, style: Option<Style>) -> Text {
                match style {
                    Some(style) => Text::styled(deck.title.as_str(), style),
                    None => Text::raw(deck.title.as_str()),
                }
            }
        }
    }
}

// Taking this code from kdheepak/taswarrior-tui because it seems to make the most sense.

use tui::backend::Backend;

pub fn draw<B>(rect: &mut Frame<B>)
where
    B: Backend,
{
    let size = rect.size();
    let background = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);
    let backing = Block::default().style(Style::default().bg(Color::Black));

    rect.render_widget(backing, background[0]);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(size);
    let top_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    let top_content = vec![Spans::from(Span::raw("Placeholder!"))];
    let bottom_content = vec![Spans::from(Span::raw("Placeholder!"))];

    rect.render_widget(Paragraph::new(top_content), top_layout[1]);
    rect.render_widget(Paragraph::new(bottom_content), top_layout[2]);
}

// TODO testing module for this part
