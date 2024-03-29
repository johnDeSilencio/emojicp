use std::error::Error;
use std::io::stdin;

use bk_tree::BKTree;
use clipboard_anywhere::set_clipboard;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use emojicp::constants::*;
use emojicp::emoji::Emoji;
use emojicp::pair::EmojiPair;
use emojicp::search::{search_exact, search_interactive};
use emojicp::{carousel, types::*};

pub fn entry(args: &Args) -> Result<(), Box<dyn Error>> {
    match search(args) {
        Ok(pair) => Ok(set_clipboard(&pair.emoji)
            .map_err(|_| Box::new(EmojiError::CannotCopyEmojiToClipboard { emoji: pair.emoji }))?),
        Err(err) => Err(err),
    }
}

pub fn search(args: &Args) -> Result<EmojiPair, Box<dyn Error>> {
    match args.description.clone() {
        Some(description) => {
            // search for emoji directly
            search_exact(description)
        }
        None => {
            // start in interactive mode
            Ok(search_interactive()?)
        }
    }
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
