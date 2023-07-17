use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crate::clipboard;
use crate::emoji::Emoji;
use crate::pair::*;
use bk_tree::BKTree;
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

enum InputMode {
    Searching,
    Selecting,
}

struct EmojiSuggestions<T>
where
    T: std::fmt::Display,
{
    bk_tree: BKTree<T>,
    state: ListState,
    items: Vec<T>,
    mode: InputMode,
    user_input: String,
    user_input_changed: bool,
    cursor_position: usize,
}

impl EmojiSuggestions<EmojiPair> {
    fn new() -> Self {
        let emoji_file = Emoji::get("static/emojitree.raw").unwrap();
        let encoded_tree = emoji_file.data.as_ref();
        let tree: BKTree<EmojiPair> = bincode::deserialize(encoded_tree).unwrap();

        EmojiSuggestions {
            bk_tree: tree,
            state: ListState::default(),
            items: Vec::new(),
            mode: InputMode::Searching,
            user_input: String::from(""),
            user_input_changed: true,
            cursor_position: 0,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i + 1 >= self.items.len() {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if 0 == i {
                    self.items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => self.items.len().saturating_sub(1),
        };

        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }

    fn select(&mut self) -> Option<&EmojiPair> {
        let Some(index) = self.state.selected() else {
            return None;
        };

        let Some(item) = self.items.get(index) else {
            return None;
        };

        println!("Found item: {}", item);
        Some(item)
    }

    fn search(&mut self, item: &String) {
        // step #1: clear current suggestions
        self.items.clear();

        // step #2: perform search on tree
        let tolerance = 10;
        let key = EmojiPair {
            description: item.clone(),
            emoji: "".to_owned(), // doesn't matter for search
        };

        let search_results = self.bk_tree.find(&key, tolerance);

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
        self.items = new_suggestions
            .iter()
            .enumerate()
            .filter(|&(_, v)| v.description.starts_with(item.as_str()))
            .map(|(_, e)| e.to_owned())
            .collect();

        // step #4: save the first 5 results
        self.items = self
            .items
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < 5)
            .map(|(_, e)| e.to_owned())
            .collect();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.user_input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character
            let before_char_to_delete = self.user_input.chars().take(from_left_to_current_index);
            // Getting all characters after the selected character
            let after_char_to_delete = self.user_input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.user_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.user_input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
}

struct App {
    items: EmojiSuggestions<EmojiPair>,
}

impl App {
    fn new() -> Self {
        App {
            items: EmojiSuggestions::new(),
        }
    }
}

pub fn ui_entry() -> Result<(), Box<dyn Error>> {
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

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();

    app.items.items.push(EmojiPair {
        description: String::from("crab"),
        emoji: String::from("🦀"),
    });
    app.items.items.push(EmojiPair {
        description: String::from("snake"),
        emoji: String::from("🐍"),
    });
    app.items.items.push(EmojiPair {
        description: String::from("monkey"),
        emoji: String::from("🐒"),
    });
    app.items.items.push(EmojiPair {
        description: String::from("eyeglasses"),
        emoji: String::from("👓"),
    });
    app.items.items.push(EmojiPair {
        description: String::from("coffee"),
        emoji: String::from("☕"),
    });

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.items.mode {
                        InputMode::Searching => match key.code {
                            KeyCode::Up => {
                                app.items.mode = InputMode::Selecting;
                                app.items.previous();
                            }
                            KeyCode::Down => {
                                app.items.mode = InputMode::Selecting;
                                app.items.next();
                            }
                            KeyCode::Backspace => {
                                app.items.delete_char();
                                let user_input = &app.items.user_input.clone();
                                app.items.search(user_input);

                                if user_input.is_empty() {
                                    app.items.items.clear();
                                }
                            }
                            KeyCode::Char('c') => {
                                if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                                    return Ok(());
                                } else {
                                    app.items.enter_char('c');
                                    let user_input = &app.items.user_input.clone();
                                    app.items.search(user_input);
                                }
                            }
                            KeyCode::Char(new_char) => {
                                app.items.enter_char(new_char);
                                let user_input = &app.items.user_input.clone();
                                app.items.search(user_input);
                            }
                            _ => {}
                        },
                        InputMode::Selecting => match key.code {
                            KeyCode::Left => app.items.unselect(),
                            KeyCode::Down => app.items.next(),
                            KeyCode::Up => app.items.previous(),
                            KeyCode::Enter => match app.items.select() {
                                Some(selection) => {
                                    clipboard::set(selection.emoji.to_owned());
                                    return Ok(());
                                }
                                None => {
                                    return Err(Box::new(
                                        crate::types::EmojiError::CannotCopyEmojiToClipboard {
                                            emoji: String::from("🦀"),
                                        },
                                    ));
                                }
                            },
                            KeyCode::Backspace => {
                                app.items.delete_char();
                                let user_input = &app.items.user_input.clone();
                                app.items.search(user_input);
                                app.items.mode = InputMode::Searching;

                                if user_input.is_empty() {
                                    app.items.items.clear();
                                }
                            }
                            KeyCode::Char('c') => {
                                if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                                    return Ok(());
                                } else {
                                    app.items.enter_char('c');
                                    let user_input = &app.items.user_input.clone();
                                    app.items.search(user_input);
                                    app.items.mode = InputMode::Searching;
                                }
                            }
                            KeyCode::Char(new_char) => {
                                app.items.mode = InputMode::Searching;
                                app.items.enter_char(new_char);

                                let user_input = &app.items.user_input.clone();
                                app.items.search(user_input);
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks, the top chunk for getting user input,
    // the bottom chunk for displaying suggestions that the user
    // can choose from:
    //
    // __Input_________________________________________________
    // |                                                      |
    // | ferris                                               |
    // |                                                      |
    // |_Suggestions__________________________________________|
    // |                                                      |
    // | 1. crab      🦀                                      |
    // | 2. snake     🐍                                      |
    // | 3. coffee    ☕                                      |
    // | ...                                                  |
    // |______________________________________________________|
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
        .split(f.size());

    // If the user input has changed, update the list
    if app.items.user_input_changed {
        // Create the input widget for searches
        let input = Paragraph::new(app.items.user_input.as_str())
            .style(match app.items.mode {
                InputMode::Searching => Style::default().fg(Color::Yellow),
                InputMode::Selecting => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(crate::constants::SEARCH_PROMPT),
            );

        // We can now render the search bar
        f.render_widget(input, chunks[0]);

        match app.items.mode {
            InputMode::Searching => f.set_cursor(
                chunks[1].x + app.items.cursor_position as u16 + 1,
                chunks[0].y + 1,
            ),
            InputMode::Selecting => {}
        }

        let msg = match app.items.mode {
            InputMode::Searching => String::from("⬆️  ⬇️  [Up / Down Arrows]: Select emoji"),
            InputMode::Selecting => {
                String::from("↩️   [Enter / Return Key]: Copy emoji to clipboard")
            }
        };

        // Create the list widget that will be used to display suggestions
        let items: Vec<ListItem> = app
            .items
            .items
            .iter()
            .map(|i| {
                let suggestion = format!("{}", i);
                ListItem::new(Line::from(suggestion)).style(Style::default())
            })
            .collect();

        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(msg.as_str())
                    .title_on_bottom(),
            )
            .style(match app.items.mode {
                InputMode::Selecting => Style::default().fg(Color::Yellow),
                InputMode::Searching => Style::default(),
            })
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        // We can now render the emoji suggestions
        f.render_stateful_widget(items, chunks[1], &mut app.items.state);
    }
}
