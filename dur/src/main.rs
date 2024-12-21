// src/main.rs
mod app;
mod scanner;
mod types;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Terminal,
};
use std::{path::PathBuf, time::Duration};
use ui::{
    draw,
    event::{handle_events, Event},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Number of threads to use for scanning
    #[arg(short, long, default_value_t = num_cpus::get())]
    threads: usize,
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(f.size());

            let path = Paragraph::new(app.current_path.to_string_lossy().into_owned())
                .style(Style::default().fg(Color::Green));
            f.render_widget(path, chunks[0]);

            if let Some(selected) = app.entries.get(app.selected) {
                draw::draw_size_bar(f, chunks[1], selected.size, app.total_size);
            }

            let mut items = draw::create_items(&app.entries, app.total_size, chunks[2].width);

            if app.others_size > 0 {
                items.push(draw::create_others_item(app.entries.len(), app.others_size));
            }

            let items = draw::create_list(items);

            f.render_stateful_widget(
                items,
                chunks[2],
                &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected)),
            );

            let help = Paragraph::new(format!(
                "Total: {}  |  ↑/↓: Navigate  ←: Back  →/Enter: Open  q: Quit",
                humansize::format_size(app.total_size, humansize::BINARY)
            ))
            .style(Style::default().fg(Color::Gray));
            f.render_widget(help, chunks[3]);
        })?;

        match handle_events(Duration::from_millis(100))? {
            Event::KeyEvent(key) => match key {
                KeyCode::Char('q') => break,
                KeyCode::Up => app.up(),
                KeyCode::Down => app.down(),
                KeyCode::Left => {
                    app.back()?;
                }
                KeyCode::Right | KeyCode::Enter => {
                    app.enter()?;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(args.path, args.threads)?;
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}
