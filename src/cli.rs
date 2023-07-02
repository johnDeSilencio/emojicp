use crate::types::*;

pub fn entry(args: &Args) -> Result<(), EmojiError> {
    println!("{:?}", args.description);
    Ok(())
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
