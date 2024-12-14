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
use std::{
    collections::HashMap,
    io::{self, stdout},
    sync::{mpsc, LazyLock, Mutex},
    thread,
    time::Duration,
};

static INITIALIZED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

#[derive(Clone)]
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
    search_tx: mpsc::Sender<String>,
    result_rx: mpsc::Receiver<Vec<String>>,
}

impl App {
    fn new() -> App {
        let (search_tx, search_rx) = mpsc::channel::<String>();
        let (result_tx, result_rx) = mpsc::channel();

        thread::spawn(move || {
            {
                let mut init = INITIALIZED.lock().unwrap();
                if !*init {
                    let _ = lookup_addresses("100");
                    let _ = search_by_address("東京");
                    *init = true;
                }
            }

            let mut last_query = String::new();
            let mut input_mode = InputMode::Postal;
            let mut cache: HashMap<String, Vec<String>> = HashMap::new();

            while let Ok(query) = search_rx.recv() {
                if query.starts_with("MODE_CHANGE:") {
                    input_mode = match &query[11..] {
                        "postal" => InputMode::Postal,
                        _ => InputMode::Address,
                    };
                    continue;
                }

                if query == last_query {
                    continue;
                }
                last_query = query.clone();

                if query.is_empty() {
                    let _ = result_tx.send(Vec::new());
                    continue;
                }

                if let Some(cached_results) = cache.get(&query) {
                    let _ = result_tx.send(cached_results.clone());
                    continue;
                }

                thread::sleep(Duration::from_millis(50));

                let results: Vec<String> = match input_mode {
                    InputMode::Postal => lookup_addresses(&query)
                        .map(|addresses| {
                            addresses
                                .into_iter()
                                .map(|addr| addr.formatted_with_kana())
                                .collect()
                        })
                        .unwrap_or_default(),
                    InputMode::Address => search_by_address(&query)
                        .into_iter()
                        .map(|addr| addr.formatted_with_kana())
                        .collect(),
                };

                cache.insert(query.clone(), results.clone());
                let _ = result_tx.send(results);
            }
        });

        App {
            input: String::new(),
            results: Vec::new(),
            input_mode: InputMode::Postal,
            scroll_state: ScrollbarState::default(),
            scroll_position: 0,
            search_tx,
            result_rx,
        }
    }

    fn search(&mut self) {
        let _ = self.search_tx.send(self.input.clone());
    }

    fn change_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
        let mode_str = match self.input_mode {
            InputMode::Postal => "postal",
            InputMode::Address => "address",
        };
        let _ = self.search_tx.send(format!("MODE_CHANGE:{}", mode_str));
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

    fn check_results(&mut self) {
        if let Ok(new_results) = self.result_rx.try_recv() {
            self.results = new_results;
            self.scroll_position = 0;
            self.scroll_state = ScrollbarState::new(self.results.len());
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
        app.check_results();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.area());

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
                    app.change_mode(match app.input_mode {
                        InputMode::Postal => InputMode::Address,
                        InputMode::Address => InputMode::Postal,
                    });
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
