use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode};
use std::time::Duration;

pub enum Event {
    KeyEvent(KeyCode),
    Tick,
}

pub fn handle_events(tick_rate: Duration) -> Result<Event> {
    if event::poll(tick_rate)? {
        if let CrosstermEvent::Key(key) = event::read()? {
            return Ok(Event::KeyEvent(key.code));
        }
    }
    Ok(Event::Tick)
}
