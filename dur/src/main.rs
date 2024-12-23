// src/main.rs
mod app;
mod scanner;
mod types;
mod ui;

use crate::app::App;
use crate::types::DirEntry;
use crate::ui::{
    draw,
    event::{handle_events, Event},
};
use anyhow::Result;
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
    widgets::{Clear, Paragraph},
    Terminal,
};
use std::{path::PathBuf, time::Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, mut app: App) -> Result<()> {
    // 初期画面をクリア
    terminal.clear()?;

    loop {
        let status_changed = app.update_status();
        let needs_refresh = app.needs_refresh();

        if status_changed || needs_refresh {
            terminal.draw(|f| {
                // 毎回画面を完全にクリア
                f.render_widget(Clear, f.size());
                let size = f.size();

                // ターミナルサイズが変更された場合のみリサイズ処理を実行
                if app.get_window_size() != Some((size.width, size.height)) {
                    app.handle_resize(size.width, size.height);
                }

                // 余白なしのレイアウトを作成
                let base_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1), // パス表示用
                        Constraint::Min(1),    // メインコンテンツ
                        Constraint::Length(2), // ステータス表示用（2行確保）
                    ])
                    .margin(0)
                    .split(size);

                // パス表示
                let path = Paragraph::new(app.current_path.to_string_lossy().into_owned())
                    .style(Style::default().fg(Color::Green));
                f.render_widget(Clear, base_chunks[0]);
                f.render_widget(path, base_chunks[0]);

                // メインコンテンツ領域
                let content_area = base_chunks[1];
                f.render_widget(Clear, content_area);

                // 表示可能な項目のみを取得
                let visible_entries: Vec<DirEntry> =
                    app.get_visible_entries().into_iter().cloned().collect();

                // リストアイテムの作成
                let mut items = draw::create_items(
                    &visible_entries,
                    app.total_size,
                    content_area.width.saturating_sub(2), // ボーダーの分を引く
                );

                // その他の項目の表示判定
                if app.others_size > 0 && app.selected >= app.scroll_offset + visible_entries.len()
                {
                    items.push(draw::create_others_item(app.entries.len(), app.others_size));
                }

                // リストの作成と表示
                let list = draw::create_list(items);
                let mut state = ratatui::widgets::ListState::default();
                state.select(Some(app.selected.saturating_sub(app.scroll_offset)));
                f.render_stateful_widget(list, content_area, &mut state);

                // ステータス領域のクリアと表示
                f.render_widget(Clear, base_chunks[2]);

                // ヘルプテキスト
                let help = Paragraph::new(format!(
                    "Total: {}  |  ↑/↓: Navigate  ←: Back  →/Enter: Open  q/ESC: Quit",
                    humansize::format_size(app.total_size, humansize::BINARY)
                ))
                .style(Style::default().fg(Color::Gray));
                f.render_widget(help, base_chunks[2]);

                // ポップアップが必要な場合のみ表示
                if app.show_popup {
                    let popup_area = draw::create_centered_rect(60, 8, size);
                    f.render_widget(Clear, popup_area);

                    let status_text = match app.get_scan_status() {
                        Some(status) => status,
                        None if app.is_scanning() => "Scanning...".to_string(),
                        _ => "".to_string(), // 完了時は何も表示しない
                    };

                    if !status_text.is_empty() {
                        let popup = draw::create_popup(&status_text);
                        f.render_widget(popup, popup_area);
                    }
                }
            })?;
        }

        match handle_events(tick_rate())? {
            Event::KeyEvent(key) => {
                match key {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.cleanup();
                        break;
                    }
                    KeyCode::Up => app.up(),
                    KeyCode::Down => app.down(),
                    KeyCode::Left => {
                        app.back()?;
                        terminal.clear()?; // バック操作時に画面をクリア
                    }
                    KeyCode::Right | KeyCode::Enter => {
                        app.enter()?;
                        terminal.clear()?; // エントリー操作時に画面をクリア
                    }
                    _ => {}
                }
            }
            Event::Resize(width, height) => {
                terminal.clear()?;
                app.handle_resize(width, height);
            }
            Event::Tick => {}
        }
    }

    Ok(())
}

fn tick_rate() -> Duration {
    Duration::from_millis(100)
}

fn main() -> Result<()> {
    let args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let app = App::new(args.path)?;
    let res = run_app(&mut terminal, app);

    // Cleanup
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
