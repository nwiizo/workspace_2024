use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub message: String,
    pub level: LogLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Error,
}

pub const LOG_FILE: &str = "ping_monitor.log";
pub const MAX_LOG_LINES: usize = 100;

pub fn write_log(message: &str, level: LogLevel) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_line = format!("[{}] [{:?}] {}\n", timestamp, level, message);

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        let _ = file.write_all(log_line.as_bytes());
    }
}
