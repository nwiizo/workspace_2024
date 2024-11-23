use reqwest::Client;
use std::time::Duration;

pub struct HttpResult {
    pub status_code: Option<u16>,
    pub response_time: Option<f64>,
    pub success: bool,
    pub error_message: Option<String>,
}

pub async fn check_http(url: &str) -> HttpResult {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .danger_accept_invalid_certs(true) // 開発用。本番環境では要検討
        .build()
        .unwrap_or_default();

    let start = std::time::Instant::now();

    match client.get(url).send().await {
        Ok(response) => {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            let status = response.status();
            HttpResult {
                status_code: Some(status.as_u16()),
                response_time: Some(elapsed),
                success: status.is_success(),
                error_message: None,
            }
        }
        Err(e) => HttpResult {
            status_code: None,
            response_time: None,
            success: false,
            error_message: Some(e.to_string()),
        },
    }
}
