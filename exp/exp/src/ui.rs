use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Row, ScrollbarState, Table, Wrap},
    Frame, Terminal,
};
use std::{io, time::Duration};
use tokio::time;

use crate::app::{App, InputMode};
use crate::logging::LogLevel;
use crate::models::Target;

pub struct LayoutConfig {
    pub left_panel_ratio: u16,
    pub top_panel_ratio: u16,
    pub monitor_scroll: usize,
    pub stats_scroll: usize,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            left_panel_ratio: 50,
            top_panel_ratio: 30,
            monitor_scroll: 0,
            stats_scroll: 0,
        }
    }
}

struct ColorPalette {
    background: Color,
    border: Color,
    header: Color,
    success: Color,
    error: Color,
    dim: Color,
    warning: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: Color::Rgb(40, 44, 52), // ダークグレー
            border: Color::Rgb(97, 175, 239),   // ブルー
            header: Color::Rgb(229, 192, 123),  // ゴールド
            success: Color::Rgb(152, 195, 121), // グリーン
            error: Color::Rgb(224, 108, 117),   // レッド
            dim: Color::Rgb(92, 99, 112),       // ダークグレー
            warning: Color::Rgb(229, 192, 123), // オレンジ
        }
    }
}

async fn read_event() -> io::Result<Event> {
    if crossterm::event::poll(Duration::from_millis(100))? {
        crossterm::event::read()
    } else {
        Ok(Event::Key(crossterm::event::KeyEvent::from(KeyCode::Null)))
    }
}

fn create_block<'a>(title: &'a str, palette: &ColorPalette) -> Block<'a> {
    Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(palette.header)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.border))
        .style(Style::default().bg(palette.background))
        .padding(Padding::new(1, 1, 0, 0))
}

fn clean_url(url: &str) -> String {
    url.replace("https://", "")
        .replace("http://", "")
        .trim_end_matches('/')
        .to_string()
}

fn draw_logs(f: &mut Frame, area: Rect, app: &App, scroll_state: &mut ScrollbarState) {
    let palette = ColorPalette::default();

    let log_messages: Vec<Line> = app
        .logs
        .iter()
        .map(|log| {
            let (color, prefix) = match log.level {
                LogLevel::Info => (palette.success, "✓"),
                LogLevel::Error => (palette.error, "✗"),
            };
            Line::from(vec![
                Span::styled(format!("{} ", prefix), Style::default().fg(color)),
                Span::styled(
                    format!("[{}] ", log.timestamp),
                    Style::default().fg(palette.dim),
                ),
                Span::styled(&log.message, Style::default().fg(color)),
            ])
        })
        .collect();

    let title = match app.input_mode {
        InputMode::Normal => "📋 LOGS",
        InputMode::Resize => "📋 LOGS (RESIZE MODE)",
    };

    let logs_block = create_block(title, &palette);

    let logs = Paragraph::new(log_messages)
        .block(logs_block)
        .wrap(Wrap { trim: true })
        .scroll((app.log_scroll_offset as u16, 0));

    let scrollbar = ratatui::widgets::Scrollbar::default()
        .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None)
        .style(Style::default().fg(palette.dim));

    f.render_widget(logs, area);
    f.render_stateful_widget(scrollbar, area, scroll_state);
}

fn draw_statistics(
    f: &mut Frame,
    area: Rect,
    app: &App,
    scroll_state: &mut ScrollbarState,
    layout_config: &LayoutConfig,
) {
    let palette = ColorPalette::default();
    let widths = &[
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(14),
    ];

    let header_cells = vec![
        "Target",
        "Total",
        "Success",
        "Failed",
        "Success Rate",
        "Min Latency",
        "Max Latency",
        "Avg Latency",
    ];

    let header = Row::new(header_cells).style(
        Style::default()
            .fg(palette.header)
            .add_modifier(Modifier::BOLD),
    );

    let visible_rows = ((area.height as usize).saturating_sub(4)).min(app.results.len());

    let stats_data: Vec<Row> = app
        .results
        .iter()
        .skip(layout_config.stats_scroll)
        .take(visible_rows)
        .map(|result| {
            let stats = &result.statistics;
            let success_rate =
                (stats.successful_checks as f64 / stats.total_checks.max(1) as f64) * 100.0;
            let row_style = if success_rate > 90.0 {
                Style::default().fg(palette.success)
            } else if success_rate > 70.0 {
                Style::default().fg(palette.warning)
            } else {
                Style::default().fg(palette.error)
            };

            let target_name = match &result.target {
                Target::Ping { host, .. } => host.clone(),
                Target::Http { url, .. } => clean_url(url),
            };

            Row::new(vec![
                target_name,
                format!("{}", stats.total_checks),
                format!("{}", stats.successful_checks),
                format!("{}", stats.failed_checks),
                format!("{:.2}%", success_rate),
                stats
                    .min_latency
                    .map_or("-".to_string(), |v| format!("{:.2}ms", v)),
                stats
                    .max_latency
                    .map_or("-".to_string(), |v| format!("{:.2}ms", v)),
                stats
                    .avg_latency
                    .map_or("-".to_string(), |v| format!("{:.2}ms", v)),
            ])
            .style(row_style)
        })
        .collect();

    let title = match app.input_mode {
        InputMode::Normal => "📊 STATISTICS",
        InputMode::Resize => "📊 STATISTICS (RESIZE MODE)",
    };

    let stats_table = Table::new(stats_data, widths)
        .header(header)
        .block(create_block(title, &palette));

    let scrollbar = ratatui::widgets::Scrollbar::default()
        .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None)
        .style(Style::default().fg(palette.dim));

    f.render_widget(stats_table, area);
    f.render_stateful_widget(scrollbar, area, scroll_state);
}

fn draw_monitor(
    f: &mut Frame,
    area: Rect,
    app: &App,
    scroll_state: &mut ScrollbarState,
    layout_config: &LayoutConfig,
) {
    let palette = ColorPalette::default();
    let header = vec![
        "Target",
        "Type",
        "Description",
        "Response Time",
        "Status",
        "Details",
        "Loss(%)",
    ];

    let header_row = Row::new(header).style(
        Style::default()
            .fg(palette.header)
            .add_modifier(Modifier::BOLD),
    );

    let status_widths = &[
        Constraint::Length(25), // Target
        Constraint::Length(8),  // Type
        Constraint::Length(25), // Description
        Constraint::Length(15), // Response Time
        Constraint::Length(10), // Status
        Constraint::Length(20), // Details
        Constraint::Length(10), // Loss(%)
    ];

    let visible_rows = ((area.height as usize).saturating_sub(4)).min(app.results.len());

    let rows: Vec<Row> = app
        .results
        .iter()
        .skip(layout_config.monitor_scroll)
        .take(visible_rows)
        .map(|result| {
            let (target, target_type, description) = match &result.target {
                Target::Ping { host, description } => (host.clone(), "PING", description.clone()),
                Target::Http {
                    url, description, ..
                } => (clean_url(url), "HTTP", description.clone()),
            };

            let status_color = if result.success {
                palette.success
            } else {
                palette.error
            };

            let status = if result.success {
                "● ALIVE"
            } else {
                "● DEAD"
            };

            let details = match &result.target {
                Target::Ping { .. } => "".to_string(),
                Target::Http { .. } => result
                    .status_code
                    .map(|code| format!("HTTP {}", code))
                    .unwrap_or_else(|| result.error_message.clone().unwrap_or_default()),
            };

            let latency = result
                .latency
                .map(|l| format!("{:.2}ms", l))
                .unwrap_or_else(|| "-".to_string());

            Row::new(vec![
                target,
                target_type.to_string(),
                description,
                latency,
                status.to_string(),
                details,
                format!("{:.2}%", result.loss_percentage()),
            ])
            .style(Style::default().fg(status_color))
        })
        .collect();

    let title = match app.input_mode {
        InputMode::Normal => "🌐 MONITOR",
        InputMode::Resize => "🌐 MONITOR (RESIZE MODE)",
    };

    let monitor_block = create_block(title, &palette);

    let table = Table::new(rows, status_widths)
        .header(header_row)
        .block(monitor_block);

    let scrollbar = ratatui::widgets::Scrollbar::default()
        .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None)
        .style(Style::default().fg(palette.dim));

    f.render_widget(table, area);
    f.render_stateful_widget(scrollbar, area, scroll_state);
}

pub async fn run_app<B: Backend>(mut app: App, terminal: &mut Terminal<B>) -> io::Result<()> {
    let ping_interval = time::interval(Duration::from_secs(1));
    let mut ping_interval = Box::pin(ping_interval);
    let mut layout_config = LayoutConfig::default();

    let mut log_scroll_state = ScrollbarState::default().content_length(app.logs.len());
    let mut monitor_scroll_state = ScrollbarState::default().content_length(app.results.len());
    let mut stats_scroll_state = ScrollbarState::default().content_length(app.results.len());

    loop {
        terminal.draw(|f| {
            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(layout_config.left_panel_ratio),
                        Constraint::Percentage(100 - layout_config.left_panel_ratio),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let right_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(layout_config.top_panel_ratio),
                        Constraint::Percentage(100 - layout_config.top_panel_ratio),
                    ]
                    .as_ref(),
                )
                .split(main_layout[1]);

            draw_monitor(
                f,
                main_layout[0],
                &app,
                &mut monitor_scroll_state,
                &layout_config,
            );
            draw_statistics(
                f,
                right_layout[0],
                &app,
                &mut stats_scroll_state,
                &layout_config,
            );
            draw_logs(f, right_layout[1], &app, &mut log_scroll_state);
        })?;

        tokio::select! {
            _ = ping_interval.tick() => {
                app.update_all_targets().await;
                app.log_scroll_offset = app.logs.len().saturating_sub(1);

                // スクロールバーの状態を更新
                log_scroll_state = log_scroll_state
                    .content_length(app.logs.len());
                monitor_scroll_state = monitor_scroll_state
                    .content_length(app.results.len());
                stats_scroll_state = stats_scroll_state
                    .content_length(app.results.len());
            }
            event = read_event() => {
                if let Ok(Event::Key(key)) = event {
                    match (key.code, key.modifiers, &app.input_mode) {
                        // リサイズモードの切り替え
                        (KeyCode::Char('w'), KeyModifiers::CONTROL, InputMode::Normal) => {
                            app.input_mode = InputMode::Resize;
                        }
                        // リサイズモードでのパネル調整
                        (KeyCode::Char('h'), KeyModifiers::NONE, InputMode::Resize) => {
                            layout_config.left_panel_ratio =
                                layout_config.left_panel_ratio.saturating_sub(5).max(20);
                        }
                        (KeyCode::Char('l'), KeyModifiers::NONE, InputMode::Resize) => {
                            layout_config.left_panel_ratio =
                                layout_config.left_panel_ratio.saturating_add(5).min(80);
                        }
                        (KeyCode::Char('j'), KeyModifiers::NONE, InputMode::Resize) => {
                            layout_config.top_panel_ratio =
                                layout_config.top_panel_ratio.saturating_sub(5).max(20);
                        }
                        (KeyCode::Char('k'), KeyModifiers::NONE, InputMode::Resize) => {
                            layout_config.top_panel_ratio =
                                layout_config.top_panel_ratio.saturating_add(5).min(70);
                        }
                        // リサイズモードのキャンセル
                        (KeyCode::Esc, _, _) => {
                            app.input_mode = InputMode::Normal;
                        }
                        // 基本操作
                        (KeyCode::Char('q'), _, _) => break,
                        (KeyCode::Tab, _, _) => {
                            app.selected_target = (app.selected_target + 1) % app.targets.len();
                        }
                        // モニターパネルのスクロール
                        (KeyCode::Up, KeyModifiers::SHIFT, _) => {
                            layout_config.monitor_scroll =
                                layout_config.monitor_scroll.saturating_sub(1);
                            monitor_scroll_state = monitor_scroll_state
                                .content_length(app.results.len())
                                .position(layout_config.monitor_scroll);
                        }
                        (KeyCode::Down, KeyModifiers::SHIFT, _) => {
                            layout_config.monitor_scroll =
                                (layout_config.monitor_scroll + 1)
                                    .min(app.results.len().saturating_sub(1));
                            monitor_scroll_state = monitor_scroll_state
                                .content_length(app.results.len())
                                .position(layout_config.monitor_scroll);
                        }
                        // 統計パネルのスクロール
                        (KeyCode::Up, KeyModifiers::CONTROL, _) => {
                            layout_config.stats_scroll =
                                layout_config.stats_scroll.saturating_sub(1);
                            stats_scroll_state = stats_scroll_state
                                .content_length(app.results.len())
                                .position(layout_config.stats_scroll);
                        }
                        (KeyCode::Down, KeyModifiers::CONTROL, _) => {
                            layout_config.stats_scroll =
                                (layout_config.stats_scroll + 1)
                                    .min(app.results.len().saturating_sub(1));
                            stats_scroll_state = stats_scroll_state
                                .content_length(app.results.len())
                                .position(layout_config.stats_scroll);
                        }
                        // ログパネルのスクロール
                        (KeyCode::Up, KeyModifiers::NONE, _) => {
                            if app.log_scroll_offset > 0 {
                                app.log_scroll_offset -= 1;
                            }
                            log_scroll_state = log_scroll_state
                                .content_length(app.logs.len())
                                .position(app.log_scroll_offset);
                        }
                        (KeyCode::Down, KeyModifiers::NONE, _) => {
                            if app.log_scroll_offset < app.logs.len().saturating_sub(1) {
                                app.log_scroll_offset += 1;
                            }
                            log_scroll_state = log_scroll_state
                                .content_length(app.logs.len())
                                .position(app.log_scroll_offset);
                        }
                        (KeyCode::PageUp, KeyModifiers::NONE, _) => {
                            app.log_scroll_offset = app.log_scroll_offset.saturating_sub(10);
                            log_scroll_state = log_scroll_state
                                .content_length(app.logs.len())
                                .position(app.log_scroll_offset);
                        }
                        (KeyCode::PageDown, KeyModifiers::NONE, _) => {
                            let max_scroll = app.logs.len().saturating_sub(1);
                            app.log_scroll_offset = (app.log_scroll_offset + 10).min(max_scroll);
                            log_scroll_state = log_scroll_state
                                .content_length(app.logs.len())
                                .position(app.log_scroll_offset);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
