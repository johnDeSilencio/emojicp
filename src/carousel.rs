use crate::clipboard;
use crate::pair::EmojiPair;
use crate::types::*;
use bk_tree::BKTree;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

// State management plan:
// #1: Two separate state objects, one for SearchState, one for SelectState
// #2: They are normal structs, not singletons, because singletons suck
// #3: ApplicationMode is still an enum with two values, either SearchMode
// or SelectMode. SearchMode takes a reference to SearchState, SelectMode
// takes a reference to SelectState (should the enum hold the state directly?)
// #4: I'd like to decouple the logic for drawing to the terminal from the
// carousel functionality itself. Is there a way I can provide the interface
// that termion expects rather than writing to the terminal directly a. la. mocks?
// #5: Rather printing directly to the terminal, I can instead pass in
// std::io::Write and write to that!
// #6: any time the underlying state of the carousel changes, the UI may update
// #7: Generally, I think it's better to put infinite loops at the topmost level
// of the app hierarchy, i.e. in main(). If I take the infinite loop of out
// Carousel, I would turn Carousel into a state machine where the methods
// in the interface would update the state machine. That means the translation
// between keyboard inputs, i.e. stdin, and calling search() would have to
// be handled by something else. That's a big match statement that should
// be covered in a function at least. Should that not be in the interface?
// I guess, someone has to handle keypresses. What is the carousel's job?
//
// 1. Allowing the user to search for items
// 2. Displaying a list of items to the user
// 3. Allowing the user to move between items
// 4. Allowing the user to select an item
//
// Given the responsibilities, I would say that it is actually not the
// responsibility of the carousel to decode keypresses. I would opt for moving
// the match statement in run() into a function that is called by main

/*
impl Carousel for EmojiCarousel {
    type Item = String;

    fn new(display: &impl std::io::Write) -> Self {
        // step #1: get the raw bytes from the embedded file
        let emoji_file = Emoji::get("static/emojitree.raw").unwrap();
        let encoded_tree = emoji_file.data.as_ref();

        // step #2: decode BKTree
        let tree: BKTree<EmojiPair> = bincode::deserialize(encoded_tree).unwrap();

        return Self {
            display,
            tree,
            mode: ApplicationMode::Search {
                search_cursor_pos: Coordinates { x: 0, y: 0 },
            },
            search_term: None,
            suggestions: vec![],
        };
    }

    fn start(&mut self) -> Result<(), DisplayError> {
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

        Ok(())
    }

    fn search(&mut self, item: &String) {
        // step #1: clear current suggestions
        self.suggestions.clear();

        // step #2: perform search on tree
        let tolerance = 5;
        let key = EmojiPair {
            description: item.clone(),
            emoji: "".to_owned(), // doesn't matter for search
        };

        let search_results = self.tree.find(&key, tolerance);

        let mut ordered_suggestions: Vec<(u32, EmojiPair)> = vec![];
        for result in search_results {
            let suggestion: EmojiPair = result.1.to_owned();
            ordered_suggestions.push((result.0, suggestion));
        }

        ordered_suggestions.sort_by_key(|k| k.0);
        let mut new_suggestions: Vec<EmojiPair> = vec![];
        for suggestion in ordered_suggestions {
            new_suggestions.push(suggestion.1);
        }

        // step #3: filter out anything that doesn't almost match
        self.suggestions = new_suggestions
            .iter()
            .enumerate()
            .filter(|&(_, v)| v.description.starts_with(item.as_str()))
            .map(|(_, e)| e.to_owned())
            .collect();

        // step #4: save the first 5 results
        self.suggestions = self
            .suggestions
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < 5)
            .map(|(_, e)| e.to_owned())
            .collect();
    }

    fn next(&mut self) {}

    fn previous(&mut self) {}

    fn select(&mut self) -> Option<&Self::Item> {
        match self.mode {
            ApplicationMode::Select {
                current_selection,
                select_cursor_pos,
            } => return self,
            ApplicationMode::Search { search_cursor_pos } => return None,
        }
    }

    fn reset(&mut self) {
        self.mode = ApplicationMode::Search {
            search_cursor_pos: Coordinates { x: 0, y: 0 },
        };
        self.search_term = None;
        self.suggestions = None;
    }
}

impl EmojiCarousel {
    pub fn new(tree: BKTree<EmojiPair>) -> Self {
        let starting_pos = Coordinates { x: 1, y: 1 };

        Self {
            tree,
            mode: UserMode::Search,
            search_term: "".to_owned(),
            current_selection: 0,
            suggestions: vec![],
            cursor_pos: starting_pos,
        }
    }

    fn init(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!(
            "{}{}{}\r",
            termion::style::Bold,
            SEARCH_PROMPT,
            termion::style::Reset
        );

        self.cursor_pos.x = SEARCH_PROMPT.len();
        self.cursor_pos.y = 1;

        for line in self.suggestions.iter() {
            println!("{}\r", line);
        }

        self.move_cursor(self.cursor_pos);
    }

    fn clear_screen(&self, stdout: &mut RawTerminal<Stdout>) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        stdout.flush().expect("stdout flushes successfully");
    }

    pub fn get_current_selection(&self) -> Option<&EmojiPair> {
        self.suggestions.get(self.current_selection as usize)
    }

    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        self.init();
        for c in stdin.keys() {
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
                        self.search_term = remove_last_char(&self.search_term);

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

    fn delete_last_char(&mut self) {
        // Move cursor back one
        if !self.search_term.is_empty() {
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
        }
    }

    fn move_cursor(&mut self, new_pos: Coordinates) {
        print!("{}", termion::cursor::Goto(new_pos.x(), new_pos.y()));
        let _ = stdout().flush();
        self.cursor_pos = new_pos;
    }

    fn move_cursor_search(&mut self) {
        self.move_cursor(Coordinates {
            x: SEARCH_PROMPT.len() + self.search_term.len(),
            y: 1,
        });
    }

    fn move_cursor_select(&mut self) {
        self.move_cursor(Coordinates { x: 1, y: 2 });
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_pos.y > 2 {
            self.cursor_pos.y -= 1;
            self.current_selection -= 1;
        }

        self.move_cursor(self.cursor_pos);
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_pos.y < self.suggestions.len() + 1 {
            self.cursor_pos.y += 1;
            self.current_selection += 1;
        }

        self.move_cursor(self.cursor_pos);
    }

    fn redraw(&mut self) {
        self.update_suggestions();
        self.draw_suggestions();
        self.move_cursor_search();
    }

    fn clear_suggestions(&mut self) {
        let old_pos = self.cursor_pos;
        self.move_cursor_select();

        for _ in &self.suggestions {
            println!("{}\r", termion::clear::CurrentLine);
        }
        self.move_cursor(old_pos);
    }

    fn update_suggestions(&mut self) {
        // step #0: clear current suggestions
        self.suggestions.clear();

        // step #1: perform search on tree
        let tolerance = 5;
        let key = EmojiPair {
            description: self.search_term.clone(),
            emoji: "".to_owned(), // doesn't matter for search
        };

        let search_results = self.tree.find(&key, tolerance);

        let mut ordered_suggestions: Vec<(u32, EmojiPair)> = vec![];
        for result in search_results {
            let suggestion: EmojiPair = result.1.to_owned();
            ordered_suggestions.push((result.0, suggestion));
        }

        ordered_suggestions.sort_by_key(|k| k.0);
        let mut new_suggestions: Vec<EmojiPair> = vec![];
        for suggestion in ordered_suggestions {
            new_suggestions.push(suggestion.1);
        }

        // step #2.5: filter out anything that doesn't almost match
        self.suggestions = new_suggestions
            .iter()
            .enumerate()
            .filter(|&(_, v)| v.description.starts_with(self.search_term.as_str()))
            .map(|(_, e)| e.to_owned())
            .collect();

        // step #3: save the first 5 results
        self.suggestions = self
            .suggestions
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < 5)
            .map(|(_, e)| e.to_owned())
            .collect();
    }

    fn draw_suggestions(&mut self) {
        // step #1: move cursor to the top of the select area
        self.move_cursor_select();

        // step #2: clear each suggestion and draw new suggestion
        for suggestion in &self.suggestions {
            print!("{}\r", termion::clear::CurrentLine);
            println!("{}", suggestion);
        }
    }
}

fn remove_last_char(search_term: &str) -> String {
    let mut chars = search_term.chars();
    chars.next_back(); // pop last char
    chars.as_str().to_string()
}

#[cfg(test)]
mod tests {
    use crate::carousel::remove_last_char;

    #[test]
    fn test_remove_last_char() {
        // empty string returns an empty string
        assert_eq!("", remove_last_char(""));

        // single character string returns empty string
        assert_eq!("", remove_last_char("&"));

        // only last character is removed from a string of characters
        assert_eq!("abc", remove_last_char("abcd"));
    }
}
*/
