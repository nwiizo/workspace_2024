use crate::http::check_http;
use crate::logging::{write_log, LogEntry, LogLevel, MAX_LOG_LINES};
use crate::models::{MonitorResult, Target};
use crate::ping::ping;
use std::collections::VecDeque;

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Resize,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}

pub struct App {
    pub targets: Vec<Target>,
    pub selected_target: usize,
    pub results: Vec<MonitorResult>,
    pub logs: VecDeque<LogEntry>,
    pub log_scroll_offset: usize,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(targets: Vec<Target>) -> Self {
        let results = targets
            .iter()
            .map(|target| MonitorResult::new(target.clone()))
            .collect();

        App {
            targets,
            selected_target: 0,
            results,
            logs: VecDeque::with_capacity(MAX_LOG_LINES),
            log_scroll_offset: 0,
            input_mode: InputMode::Normal, // ここを修正：InputMode::Normalを指定
        }
    }

    pub fn add_log(&mut self, message: String, level: LogLevel) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = LogEntry {
            timestamp: timestamp.clone(),
            message: message.clone(),
            level,
        };

        if self.logs.len() >= MAX_LOG_LINES {
            self.logs.pop_front();
        }
        self.logs.push_back(log_entry);

        write_log(&message, level);
    }

    pub fn update_result(
        &mut self,
        index: usize,
        success: bool,
        latency: Option<f64>,
        status_code: Option<u16>,
        error_message: Option<String>,
    ) {
        if let Some(result) = self.results.get_mut(index) {
            result.timestamp = std::time::Instant::now();
            result.success = success;
            result.latency = latency;
            result.total_count += 1;
            result.status_code = status_code;
            result.error_message = error_message;

            if !success {
                result.loss_count += 1;
            }
            result.statistics.update(success, latency);

            let target_info = match &result.target {
                Target::Ping { host, .. } => format!("Host: {}", host),
                Target::Http { url, .. } => format!("URL: {}", url),
            };

            let status_info = if let Some(code) = status_code {
                format!("Status: {}", code)
            } else {
                "Status: N/A".to_string()
            };

            let log_message = format!(
                "{} - {} - Latency: {} - {}",
                target_info,
                if success { "Success" } else { "Failed" },
                latency.map_or("-".to_string(), |l| format!("{:.2}ms", l)),
                status_info
            );

            self.add_log(
                log_message,
                if success {
                    LogLevel::Info
                } else {
                    LogLevel::Error
                },
            );
        }
    }

    pub async fn update_all_targets(&mut self) {
        let mut updates = Vec::new();
        for (index, target) in self.targets.iter().enumerate() {
            match target {
                Target::Ping { host, .. } => {
                    let (success, latency) = ping(host).await;
                    updates.push((index, success, latency, None, None));
                }
                Target::Http {
                    url,
                    expected_status,
                    ..
                } => {
                    let result = check_http(url).await;
                    let success = match expected_status {
                        Some(expected) => result.status_code == Some(*expected),
                        None => result.success,
                    };
                    updates.push((
                        index,
                        success,
                        result.response_time,
                        result.status_code,
                        result.error_message,
                    ));
                }
            }
        }

        for (index, success, latency, status_code, error_message) in updates {
            self.update_result(index, success, latency, status_code, error_message);
        }
    }
}
