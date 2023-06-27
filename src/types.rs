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

pub trait EmojiCarousel {
    pub fn new(tree: BKTree<EmojiPair>) -> Self;

    pub fn init(&mut self);

    pub fn reset(&mut self, stdout: &mut RawTerminal<Stdout>);

    pub fn up(&mut self);

    pub fn down(&mut self);
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
