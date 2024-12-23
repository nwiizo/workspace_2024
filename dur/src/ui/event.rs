// src/ui/event.rs
use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEventKind};
use std::time::Duration;

#[derive(Debug)]
pub enum Event {
    KeyEvent(KeyCode),
    Resize(u16, u16),
    Tick,
}

pub fn handle_events(tick_rate: Duration) -> Result<Event> {
    if event::poll(tick_rate)? {
        match event::read()? {
            CrosstermEvent::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    return Ok(Event::KeyEvent(key.code));
                }
            }
            CrosstermEvent::Resize(width, height) => {
                return Ok(Event::Resize(width, height));
            }
            _ => {}
        }
    }
    Ok(Event::Tick)
}
