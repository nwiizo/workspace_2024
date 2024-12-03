use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use jpostcode_rs::{lookup_addresses, search_by_address};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, ScrollbarState},
    Terminal,
};
use std::io::{self, stdout};

enum InputMode {
    Postal,
    Address,
}

struct App {
    input: String,
    results: Vec<String>,
    input_mode: InputMode,
    scroll_state: ScrollbarState,
    scroll_position: u16,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            results: Vec::new(),
            input_mode: InputMode::Postal,
            scroll_state: ScrollbarState::default(),
            scroll_position: 0,
        }
    }

    fn search(&mut self) {
        self.results.clear();
        if self.input.is_empty() {
            return;
        }

        match self.input_mode {
            InputMode::Postal => {
                if let Ok(addresses) = lookup_addresses(&self.input) {
                    for addr in addresses {
                        self.results.push(addr.formatted_with_kana());
                    }
                }
            }
            InputMode::Address => {
                let addresses = search_by_address(&self.input);
                for addr in addresses {
                    self.results.push(addr.formatted_with_kana());
                }
            }
        }
        self.scroll_position = 0;
        self.scroll_state = ScrollbarState::new(self.results.len());
    }

    fn scroll_up(&mut self) {
        self.scroll_position = self.scroll_position.saturating_sub(1);
    }

    fn scroll_down(&mut self) {
        if !self.results.is_empty() {
            self.scroll_position = self
                .scroll_position
                .saturating_add(1)
                .min((self.results.len() as u16).saturating_sub(1));
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.size());

            let mode = match app.input_mode {
                InputMode::Postal => "郵便番号検索 (Tab: モード切替, ↑↓: スクロール, Esc: 終了)",
                InputMode::Address => "住所検索 (Tab: モード切替, ↑↓: スクロール, Esc: 終了)",
            };

            let input_block = Block::default().title(mode).borders(Borders::ALL);
            let input = Paragraph::new(app.input.as_str())
                .block(input_block)
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(input, chunks[0]);

            let results_text = if app.results.is_empty() {
                "検索結果がありません".to_string()
            } else {
                app.results.join("\n")
            };

            let results_block = Block::default()
                .title(format!("検索結果 ({} 件)", app.results.len()))
                .borders(Borders::ALL);
            let results = Paragraph::new(results_text)
                .block(results_block)
                .scroll((app.scroll_position, 0));
            f.render_widget(results, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                    app.search();
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    app.search();
                }
                KeyCode::Up => app.scroll_up(),
                KeyCode::Down => app.scroll_down(),
                KeyCode::Tab => {
                    app.input_mode = match app.input_mode {
                        InputMode::Postal => InputMode::Address,
                        InputMode::Address => InputMode::Postal,
                    };
                    app.input.clear();
                    app.results.clear();
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
