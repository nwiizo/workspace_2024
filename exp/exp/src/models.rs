use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Target {
    #[serde(rename = "ping")]
    Ping { host: String, description: String },
    #[serde(rename = "http")]
    Http {
        url: String,
        description: String,
        expected_status: Option<u16>,
    },
}

#[derive(Debug, Clone)]
pub struct Statistics {
    pub total_checks: u64,
    pub successful_checks: u64,
    pub failed_checks: u64,
    pub min_latency: Option<f64>,
    pub max_latency: Option<f64>,
    pub avg_latency: Option<f64>,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            total_checks: 0,
            successful_checks: 0,
            failed_checks: 0,
            min_latency: None,
            max_latency: None,
            avg_latency: None,
        }
    }

    pub fn update(&mut self, success: bool, latency: Option<f64>) {
        self.total_checks += 1;
        if success {
            self.successful_checks += 1;
            if let Some(lat) = latency {
                match (self.min_latency, self.max_latency) {
                    (None, None) => {
                        self.min_latency = Some(lat);
                        self.max_latency = Some(lat);
                        self.avg_latency = Some(lat);
                    }
                    (Some(min), Some(max)) => {
                        self.min_latency = Some(min.min(lat));
                        self.max_latency = Some(max.max(lat));
                        self.avg_latency = Some(
                            (self.avg_latency.unwrap() * (self.successful_checks - 1) as f64 + lat)
                                / self.successful_checks as f64,
                        );
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            self.failed_checks += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonitorResult {
    pub timestamp: Instant,
    pub target: Target,
    pub latency: Option<f64>,
    pub success: bool,
    pub loss_count: u32,
    pub total_count: u32,
    pub statistics: Statistics,
    pub status_code: Option<u16>,
    pub error_message: Option<String>,
}

impl MonitorResult {
    pub fn new(target: Target) -> Self {
        MonitorResult {
            timestamp: Instant::now(),
            target,
            latency: None,
            success: false,
            loss_count: 0,
            total_count: 0,
            statistics: Statistics::new(),
            status_code: None,
            error_message: None,
        }
    }

    pub fn loss_percentage(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            (self.loss_count as f64 / self.total_count as f64) * 100.0
        }
    }
}
