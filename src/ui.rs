use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

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

struct EmojiSuggestions<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> EmojiSuggestions<T> {
    fn new() -> Self {
        EmojiSuggestions {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    fn next(&mut self) {}

    fn previous(&mut self) {}

    fn unselect(&mut self) {}

    fn select(&mut self) {}
}

struct App<'a> {
    user_input: &'a str,
    user_input_changed: bool,
    items: EmojiSuggestions<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        App {
            user_input: "",
            user_input_changed: true,
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
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    app.items.items.push(("Lorem ipsum", 0));
    app.items.items.push(("Pickle rick", 2));
    app.items.items.push(("Please work", 153));

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => app.items.unselect(),
                        KeyCode::Down => app.items.next(),
                        KeyCode::Up => app.items.previous(),
                        _ => {}
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
    // | 1. 🦀                                                |
    // | 2. 🐍                                                |
    // | 3. ☕                                                |
    // | ...                                                  |
    // |______________________________________________________|
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
        .split(f.size());

    // If the user input has changed, update the list
    if app.user_input_changed {
        // Create the input widget for searches
        let input = Paragraph::new(app.user_input).block(
            Block::default()
                .borders(Borders::ALL)
                .title(crate::constants::SEARCH_PROMPT),
        );

        // We can now render the search bar
        f.render_widget(input, chunks[0]);

        // Create the list widget that will be used to display suggestions
        let items: Vec<ListItem> = app
            .items
            .items
            .iter()
            .map(|i| ListItem::new(Line::from(i.0)).style(Style::default()))
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Suggestions"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        // We can now render the emoji suggestions
        f.render_stateful_widget(items, chunks[1], &mut app.items.state);
    }
}
