use crate::clipboard::set_clipboard;
use bk_tree::BKTree;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub description: String,
    pub emoji: String,
}

impl fmt::Display for Suggestion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}", self.description, self.emoji)
    }
}

impl AsRef<str> for Suggestion {
    fn as_ref(&self) -> &str {
        &self.description
    }
}

struct Suggestions {
    lines: Vec<Suggestion>,
}

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

#[derive(Debug)]
struct TerminalSize {
    pub width: usize,
    pub height: usize,
}

pub trait Carousel {
    fn new() -> Self;
    fn run(&mut self);
}

pub struct EmojiCarousel {
    // The BKTree used for fuzzy searches
    tree: BKTree<Suggestion>,

    // The number of rows and columns in the terminal
    terminal_size: TerminalSize,

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
    suggestions: Suggestions,

    // The actual position of the terminal cursor
    cursor_pos: Coordinates,

    // The location in the terminal the cursor
    // needs to be to continue typing search characters
    search_pos: Coordinates,

    // The location in the terminal the cursor
    // needs to be to select the first option from
    // the list of currently displayed suggestions
    select_pos: Coordinates,
}

impl EmojiCarousel {
    pub fn new(tree: BKTree<Suggestion>) -> Self {
        let size = termion::terminal_size().expect("termion gets the # of terminal rows & columns");

        let starting_pos = Coordinates { x: 1, y: 1 };

        Self {
            tree: tree,
            terminal_size: TerminalSize {
                width: size.0 as usize,
                height: size.1 as usize,
            },
            mode: UserMode::Search,
            search_term: "".to_owned(),
            current_selection: 0,
            suggestions: Suggestions { lines: vec![] },
            search_pos: starting_pos,
            select_pos: starting_pos,
            cursor_pos: starting_pos,
        }
    }

    fn show(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        let search_prompt = "Emoji you are looking for: ";
        println!("{}\r", search_prompt);

        self.search_pos.x += search_prompt.len();
        self.search_pos.y = 1;
        self.select_pos.x = 1;
        self.select_pos.y = 2;
        self.cursor_pos = self.search_pos;

        for line in self.suggestions.lines.iter() {
            println!("{}\r", line);
        }

        self.move_cursor(self.cursor_pos);
    }

    pub fn get_current_selection(&self) -> Option<&Suggestion> {
        self.suggestions.lines.get(self.current_selection as usize)
    }

    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        self.show();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('c') => {
                    break;
                }
                Key::Char('\n') => {
                    if self.mode == UserMode::Select {
                        set_clipboard(self.get_current_selection().unwrap().emoji.to_string());
                        print!("{}{}\r", termion::clear::All, termion::cursor::Goto(1, 1));
                        stdout.flush();
                        break;
                    } else {
                        self.update_suggestions();
                        self.draw_suggestions();
                        self.move_cursor_search();
                    }
                }
                Key::Backspace => {
                    if self.search_term.len() > 0 {
                        self.mode = UserMode::Search;
                        self.move_cursor_search();
                        self.delete_last_char();
                        self.search_term = remove_last_char(&self.search_term);

                        if self.search_term.is_empty() {
                            self.clear_suggestions();
                        } else {
                            self.update_suggestions();
                            self.draw_suggestions();
                            self.move_cursor_search();
                        }
                    }
                }
                Key::Up => {
                    if self.mode == UserMode::Select {
                        self.move_cursor_select();
                        self.move_cursor_up();
                    }
                }
                Key::Down => {
                    if self.mode == UserMode::Search {
                        self.mode = UserMode::Select;
                        self.move_cursor_select();
                    } else {
                        self.move_cursor_down();
                    }
                }
                Key::Char(typed_char) => {
                    self.mode = UserMode::Search;
                    self.move_cursor_search();
                    print!("{}", typed_char);
                    self.search_term += &typed_char.to_string();

                    // Update tracking of cursor
                    self.cursor_pos.x += 1;
                    self.search_pos.x += 1;

                    self.update_suggestions();
                    self.draw_suggestions();
                    self.move_cursor_search();
                }
                _ => {} // do nothing for other keys
            }
            stdout.flush().expect("stdout flushes successfully");
        }
    }

    fn delete_last_char(&mut self) {
        // Move cursor back one
        if self.search_term.len() > 0 {
            let back_one = Coordinates {
                x: self.cursor_pos.x - 1,
                y: self.cursor_pos.y,
            };
            self.move_cursor(back_one);

            // Print space to clear that character
            print!(" ");

            // Move cursor back one again
            self.move_cursor(back_one);

            // Update member variable
            self.cursor_pos.x = back_one.x;
            self.search_pos = self.cursor_pos
        }
    }

    fn move_cursor(&mut self, new_pos: Coordinates) {
        print!("{}", termion::cursor::Goto(new_pos.x(), new_pos.y()));
        stdout().flush();
        self.cursor_pos = new_pos;
    }

    fn move_cursor_search(&mut self) {
        self.move_cursor(self.search_pos);
    }

    fn move_cursor_select(&mut self) {
        self.move_cursor(self.select_pos);
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_pos.y > 2 {
            self.cursor_pos.y -= 1;
            self.select_pos.y -= 1;
            self.current_selection -= 1;
        }

        self.move_cursor(self.cursor_pos);
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_pos.y < self.suggestions.lines.len() + 1 {
            self.cursor_pos.y += 1;
            self.select_pos.y += 1;
            self.current_selection += 1;
        }

        self.move_cursor(self.cursor_pos);
    }

    fn clear_suggestions(&mut self) {
        let old_pos = self.cursor_pos;
        self.move_cursor_select();
        for _ in &self.suggestions.lines {
            println!("{}\r", termion::clear::CurrentLine);
        }
        self.move_cursor(old_pos);

        // step #2: clear current suggestions
        self.suggestions.lines.clear();
    }

    fn update_suggestions(&mut self) {
        // step #1: perform search on tree
        let tolerance = 5;
        let key = Suggestion {
            description: self.search_term.clone(),
            emoji: "".to_owned(), // doesn't matter for search
        };

        // step #2: clear screen and member variable suggestions
        self.clear_suggestions();

        let search_results = self.tree.find(&key, tolerance);

        let mut ordered_suggestions: Vec<(u32, Suggestion)> = vec![];
        for result in search_results {
            let suggestion: Suggestion = result.1.to_owned();
            ordered_suggestions.push((result.0, suggestion));
        }

        ordered_suggestions.sort_by_key(|k| k.0);
        let mut new_suggestions: Vec<Suggestion> = vec![];
        for suggestion in ordered_suggestions {
            new_suggestions.push(suggestion.1);
        }

        // step #3: save results
        self.suggestions.lines = new_suggestions
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < 5)
            .map(|(_, e)| e.to_owned())
            .collect();
    }

    fn draw_suggestions(&mut self) {
        // step #1: move cursor to search area
        self.move_cursor_select();

        // step #2: clear all lines so far and draw the new lines
        for suggestion in &self.suggestions.lines {
            print!("{}\r", termion::clear::CurrentLine);
            println!("{}", suggestion)
        }
    }
}

fn remove_last_char(search_term: &str) -> String {
    let mut chars = search_term.chars();
    chars.next_back(); // pop last char
    chars.as_str().to_string()
}
