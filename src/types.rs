use crate::pair::EmojiPair;
use bk_tree::BKTree;
use termion::raw::RawTerminal;

// What mode the application is in
#[derive(PartialEq)]
enum UserMode {
    // The user is typing characters to
    // search for an emoji
    Search,

    // The user is selecting the desired emoji
    // from the displayed options
    Select,
}

// Where the cursor is in the terminal
#[derive(Debug, Clone, Copy)]
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

// `Carousel` trait is a generic interface for allowing the user to search
// for an item, presenting the user with a list of items, and allowing the
// user to select an item from that list.
pub trait Carousel {
    // The type of data that the user can search for and select from the `Carousel`
    type Item;

    // `new` returns a new instance of the `Carousel` type. Note that the
    // `Carousel` will not start displaying items or allow the user to search
    // for or select an item until `start` has been called
    fn new() -> Self;

    // `start` initializes the `Carousel` object, allowing the user to
    // start searching for and selecting items from the `Carousel`. Calling
    // `start` rotates the `Carousel` to the default location
    fn start(&mut self);

    // `search` the `Carousel` for `item`. Calling `search` updates the list
    // of items displayed to the user with the items that most closely match
    // `item`. Calling `search` also rotates the `Carousel` to the default
    // location
    fn search(&mut self, item: &Self::Item);

    // `next` rotates the `Carousel` to the next item in the list presented
    // to the user. If the `Carousel` has already been rotated to the end
    // of the list, then calling this method does nothing
    fn next(&mut self);

    // `previous` rotates the `Carousel` to the previous item in the list
    // presented to the user. If the `Carousel` has already been rotated to
    // the beginning of the list, then calling this method does nothing
    fn previous(&mut self);

    // `select` returns the item currently selected in the list of items
    // presented to the user
    fn select(&mut self) -> &Self::Item;

    // `reset` clears the user's search and resets the `Carousel` to its
    // initial state. Note that `start` must be called after calling `reset`
    // to display items or allow the user to search for or select an item again.
    fn reset(&mut self);
}

pub struct EmojiCarousel {
    // The BKTree used for fuzzy searches
    tree: BKTree<EmojiPair>,

    // What mode the application is in, i.e. if the user
    // is typing to perform a search or selecting an emoji
    // from the available options
    mode: UserMode,

    // The characters the user has typed in thusfar
    search_term: String,

    // The currently selected emoji from the list
    current_selection: u8,

    // List of suggested emojis and their names
    // currently being shown to the user
    suggestions: Vec<EmojiPair>,

    // The current position of the terminal cursor
    cursor_pos: Coordinates,
}
