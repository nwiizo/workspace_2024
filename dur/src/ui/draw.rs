// src/ui/draw.rs
use crate::types::DirEntry;
use humansize::{format_size, BINARY};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use unicode_width::UnicodeWidthStr;

fn get_bar_style(percentage: u16) -> Style {
    match percentage {
        76..=100 => Style::default().fg(Color::Red),
        51..=75 => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::Green),
    }
}

fn create_bar(percentage: u16) -> (String, Style) {
    let width = 20;
    let filled = ((percentage as f64 / 100.0) * width as f64).round() as usize;
    let empty = width - filled;
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    (bar, get_bar_style(percentage))
}

pub fn draw_entry(entry: &DirEntry, total_size: u64, width: u16) -> Line<'static> {
    let name = entry
        .path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "???".to_string());

    let size = format_size(entry.size, BINARY);
    let percentage = if total_size > 0 {
        ((entry.size as f64 / total_size as f64) * 100.0) as u16
    } else {
        0
    };

    let prefix = if entry.is_dir {
        if entry.children_count > 0 {
            format!("📁 ({} items) ", entry.children_count)
        } else {
            "📁 ".to_string()
        }
    } else {
        "📄 ".to_string()
    };

    let (bar, bar_style) = create_bar(percentage);

    let name_width = width.saturating_sub(prefix.width() as u16 + bar.width() as u16 + 30) as usize;
    let truncated_name = if name.width() > name_width {
        format!("{}...", &name[..name_width.saturating_sub(3)])
    } else {
        format!("{:name_width$}", name)
    };

    let spans = vec![
        Span::from(prefix),
        Span::from(truncated_name),
        Span::raw(" "),
        Span::styled(bar, bar_style),
        Span::styled(format!(" {:>8} ", size), Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("({:>3}%)", percentage),
            Style::default().fg(Color::Cyan),
        ),
    ];

    Line::from(spans)
}

pub fn create_items(entries: &[DirEntry], total_size: u64, width: u16) -> Vec<ListItem<'static>> {
    entries
        .iter()
        .map(|entry| ListItem::new(draw_entry(entry, total_size, width)))
        .collect()
}

pub fn create_list<'a>(items: Vec<ListItem<'a>>) -> List<'a> {
    List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}

pub fn create_others_item(entries_count: usize, others_size: u64) -> ListItem<'static> {
    let size = format_size(others_size, BINARY);
    let name = format!("... and {} other items", entries_count);

    let spans = vec![
        Span::from("📄 "),
        Span::from(name),
        Span::raw(" "),
        Span::styled(format!(" {:>8} ", size), Style::default().fg(Color::Yellow)),
    ];

    ListItem::new(Line::from(spans))
}

pub fn create_centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn create_popup(status: &str) -> Paragraph<'static> {
    Paragraph::new(status.to_string())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Scanning Status")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow)),
        )
}
