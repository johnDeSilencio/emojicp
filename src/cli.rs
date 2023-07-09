use bk_tree::BKTree;
use clipboard_anywhere::set_clipboard;
use termion::raw::IntoRawMode;

use crate::constants::*;
use crate::emoji::Emoji;
use crate::pair::EmojiPair;
use crate::{carousel, types::*};

pub fn entry(args: &Args) -> Result<(), EmojiError> {
    match search(args) {
        Ok(pair) => set_clipboard(&pair.emoji)
            .map_err(|_| EmojiError::CannotCopyEmojiToClipboard { emoji: pair.emoji }),
        Err(err) => Err(err),
    }
}

pub fn search(args: &Args) -> Result<EmojiPair, EmojiError> {
    match args.description.clone() {
        Some(description) => {
            // search for emoji directly
            search_exact(description)
        }
        None => {
            // start in interactive mode
            search_interactive()
        }
    }
}

fn search_exact(description: String) -> Result<EmojiPair, EmojiError> {
    // Get the raw bytes from the embedded file
    let emoji_file = Emoji::get(EMOJI_TREE_FILE).ok_or(EmojiError::IoError {
        filename: String::from(EMOJI_TREE_FILE),
    })?;
    let encoded_tree = emoji_file.data.as_ref();

    // Decode the BKTree
    let tree: BKTree<EmojiPair> =
        bincode::deserialize(encoded_tree).map_err(|_| EmojiError::CannotDeserializeBKTree {
            filename: String::from(EMOJI_TREE_FILE),
        })?;

    // Search the BKTree for the emoji
    tree.find_exact(&EmojiPair {
        description: description.clone(),
        emoji: String::from(""), // doesn't matter for the search
    })
    .ok_or(EmojiError::InvalidEmojiName { description })
    .cloned()
}

fn search_interactive() -> Result<EmojiPair, EmojiError> {
    let mut stdout = std::io::stdout();
    stdout
        .into_raw_mode()
        .map_err(|_| EmojiError::CannotEnterRawMode)?;

    /*
    let carousel = EmojiCarousel::new(stdout);

    for key in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => {}
            Key::Char('\n') => {}
            Key::Backspace => {}
            Key::Up => {}
            Key::Down => {}
            Key::Char(typed_char) => {}
            _ => {} // do nothing for other keys
        }
    }
    */
    Ok(EmojiPair {
        description: String::from(""),
        emoji: String::from(""),
    })
}

/*
fn start_carousel() {
    let mut stdout = self.display.into_raw_mode()?;

    for key in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => {
                self.clear_screen(&mut stdout);
                break;
            }
            Key::Char('\n') => match self.mode {
                UserMode::Search => {} // do nothing
                UserMode::Select => {
                    clipboard::set(self.get_current_selection().unwrap().emoji.to_string());
                    print!("{}{}\r", termion::clear::All, termion::cursor::Goto(1, 1));
                    let _ = stdout.flush();
                    break;
                }
            },
            Key::Backspace => {
                if !self.search_term.is_empty() {
                    self.mode = UserMode::Search;
                    self.move_cursor_search();
                    self.delete_last_char();
                    self.search_term = Some(remove_last_char(&self.search_term));

                    if self.search_term.is_empty() {
                        self.clear_suggestions();
                        self.suggestions.clear();
                    } else {
                        self.redraw();
                    }
                }
            }
            Key::Up => match self.mode {
                UserMode::Search => {}
                UserMode::Select => {
                    if !self.suggestions.is_empty() {
                        self.move_cursor_up();
                    }
                }
            },
            Key::Down => match self.mode {
                UserMode::Search => {
                    if !self.suggestions.is_empty() {
                        self.mode = UserMode::Select;
                        self.move_cursor_select();
                    }
                }
                UserMode::Select => {
                    if !self.suggestions.is_empty() {
                        self.move_cursor_down();
                    }
                }
            },
            Key::Char(typed_char) => {
                self.mode = UserMode::Search;
                self.move_cursor_search();
                print!("{}", typed_char);
                self.search_term += &typed_char.to_string();

                // Update tracking of cursor
                self.cursor_pos.x += 1;

                self.redraw();
            }
            _ => {} // do nothing for other keys
        }
    }
}
*/
