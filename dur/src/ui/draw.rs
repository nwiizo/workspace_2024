use crate::types::DirEntry;
use humansize::{format_size, BINARY};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn draw_entry(entry: &DirEntry, total_size: u64, width: u16) -> Line<'static> {
    let name = entry
        .path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "???".to_string());

    let size = format_size(entry.size, BINARY);
    let percentage = if total_size > 0 {
        (entry.size as f64 / total_size as f64 * 100.0) as u16
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

    let available_width = width.saturating_sub(prefix.width() as u16 + 20) as usize;
    let truncated_name = if name.width() > available_width {
        format!("{}...", &name[..available_width.saturating_sub(3)])
    } else {
        format!("{:width$}", name, width = available_width)
    };

    Line::from(vec![
        Span::from(prefix),
        Span::from(truncated_name),
        Span::styled(format!(" {:>8} ", size), Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("({:>3}%)", percentage),
            Style::default().fg(Color::Cyan),
        ),
    ])
}

pub fn draw_size_bar(f: &mut Frame, area: Rect, size: u64, total_size: u64) {
    let percentage = if total_size > 0 {
        (size as f64 / total_size as f64 * 100.0) as u16
    } else {
        0
    };

    let label = format!(
        "{} / {} ({}%)",
        format_size(size, BINARY),
        format_size(total_size, BINARY),
        percentage
    );

    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(
            Style::default()
                .fg(Color::Blue)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .label(label)
        .ratio(percentage as f64 / 100.0);

    f.render_widget(gauge, area);
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
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}

pub fn create_others_item(entries_count: usize, others_size: u64) -> ListItem<'static> {
    ListItem::new(Line::from(vec![
        Span::from("... "),
        Span::styled(
            format!("{} other items", entries_count),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!(" ({}) ", format_size(others_size, BINARY)),
            Style::default().fg(Color::DarkGray),
        ),
    ]))
}
