use std::error::Error;
use std::io::stdin;

use bk_tree::BKTree;
use clipboard_anywhere::set_clipboard;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::constants::*;
use crate::emoji::Emoji;
use crate::pair::EmojiPair;
use crate::ui::{run_app, App};
use crate::{carousel, types::*};

use std::{
    io,
    time::{Duration, Instant},
};

use crate::clipboard;
use crate::pair::*;
use crate::types::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::*;
use ratatui::{
    backend::{self, Backend},
    buffer::{self, Buffer},
    layout::{self, Alignment, Constraint, Corner, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, *},
    symbols::{self, Marker},
    terminal::{self, Frame, Terminal, TerminalOptions, Viewport},
    text::{self, Line, Masked, Span, Text},
};

pub fn search_exact(description: String) -> Result<EmojiPair, Box<dyn Error>> {
    // Get the raw bytes from the embedded file
    let emoji_file = Emoji::get(EMOJI_TREE_FILE).ok_or(Box::new(EmojiError::IoError {
        filename: String::from(EMOJI_TREE_FILE),
    }))?;
    let encoded_tree = emoji_file.data.as_ref();

    // Decode the BKTree
    let tree: BKTree<EmojiPair> = bincode::deserialize(encoded_tree).map_err(|_| {
        Box::new(EmojiError::CannotDeserializeBKTree {
            filename: String::from(EMOJI_TREE_FILE),
        })
    })?;

    // Search the BKTree for the emoji
    Ok(tree
        .find_exact(&EmojiPair {
            description: description.clone(),
            emoji: String::from(""), // doesn't matter for the search
        })
        .ok_or(Box::new(EmojiError::InvalidEmojiName { description }))
        .cloned()?)
}

pub fn search_interactive() -> Result<EmojiPair, Box<dyn Error>> {
    // Initialize terminal for interactive environment
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // Restore terminal to normal mode
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    res
}
