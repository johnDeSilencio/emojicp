use std::default;

use crate::constants::ABOUT_DESCRIPTION;
use crate::emoji::Emoji;
use crate::pair::EmojiPair;
use bk_tree::BKTree;
use clap::Parser;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use thiserror::Error;

#[derive(Parser)]
#[command(version)]
#[command(about = ABOUT_DESCRIPTION, long_about=None)]
pub struct Args {
    pub description: Option<String>,
}

// `Carousel` trait is a generic interface for allowing the user to search for
// an item, presenting the user with a list of items, and allowing the user to
// select an item from that list.
pub trait Carousel {
    // The type of data that the user can search for and select from the
    // `Carousel`
    type Item;

    // `new` returns a new instance of the `Carousel` type. The list of items
    // will be displayed to the user by writing to `display`. In a terminal
    // environment, `display` will typically be `std::io::stdout`. Note that the
    // `Carousel` will not start displaying items or allow the user to search
    // for or select an item until `start` has been called
    fn new(display: &impl std::io::Write) -> Self;

    // `start` initializes the `Carousel` object, allowing the user to start
    // searching for and selecting items from the `Carousel`. Calling `start`
    // rotates the `Carousel` to the default location
    fn start(&mut self);

    // `search` the `Carousel` for `item`. Calling `search` updates the list of
    // items displayed to the user with the items that most closely match
    // `item`. Calling `search` also rotates the `Carousel` to the default
    // location
    fn search(&mut self, item: &Self::Item);

    // `next` rotates the `Carousel` to the next item in the list presented to
    // the user. If the `Carousel` has already been rotated to the end of the
    // list, then calling this method does nothing
    fn next(&mut self);

    // `previous` rotates the `Carousel` to the previous item in the list
    // presented to the user. If the `Carousel` has already been rotated to the
    // beginning of the list, then calling this method does nothing
    fn previous(&mut self);

    // `select` returns the item currently selected in the list of items
    // presented to the user
    fn select(&mut self) -> Option<&Self::Item>;

    // `quit` clears writer and exits the loop started when `search` was called
    fn quit(&mut self);
}

// Where the cursor is in the terminal
#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Coordinates {
    fn x(&self) -> u16 {
        self.x as u16
    }

    fn y(&self) -> u16 {
        self.y as u16
    }
}

// What mode the application is in
#[derive(PartialEq)]
pub enum ApplicationMode {
    // The user is typing characters to search for an emoji
    Search {
        // The position of the cursor when the user is searching
        // for an emoji
        search_cursor_pos: Coordinates,
    },

    // The user is selecting the desired emoji from the displayed options
    Select {
        // The currently selected emoji from the list
        current_selection: u8,

        // The position of the cursor when the user is selecting
        // an emoji from the list
        select_cursor_pos: Coordinates,
    },
}

pub struct EmojiCarousel<W>
where
    W: std::io::Write,
{
    // The interface that is written to in order
    // to display each emoji and its description
    pub display: W,

    // The BKTree used for fuzzy searches
    pub tree: BKTree<EmojiPair>,

    // What mode the application is in, i.e. if the user
    // is typing to perform a search or selecting an emoji
    // from the available options
    pub mode: ApplicationMode,

    // The characters the user has typed in thusfar. If
    // `None`, then the user has yet to start searchng
    // or the carousel has been reset
    pub search_term: Option<String>,

    // List of suggested emojis and their names
    // currently being shown to the user
    pub suggestions: Vec<EmojiPair>,
}

#[derive(Error, Debug)]
pub enum EmojiError {
    #[error("supplied writer cannot enter raw mode")]
    CannotEnterRawMode,

    #[error("cannot open file `{filename:?}`")]
    IoError { filename: String },

    #[error("cannot deserialize BKTree from file `{filename:?}`")]
    CannotDeserializeBKTree { filename: String },

    #[error("cannot find the emoji `{description:?}`")]
    InvalidEmojiName { description: String },
}
