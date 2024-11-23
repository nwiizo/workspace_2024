use std::time::Instant;
use tokio::process::Command;

pub async fn ping(target: &str) -> (bool, Option<f64>) {
    let start = Instant::now();

    let output = if cfg!(target_os = "windows") {
        Command::new("ping")
            .arg("-n")
            .arg("1")
            .arg(target)
            .output()
            .await
    } else {
        Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(target)
            .output()
            .await
    };

    match output {
        Ok(output) => {
            let success = output.status.success();
            let latency = if success {
                Some(start.elapsed().as_secs_f64() * 1000.0)
            } else {
                None
            };
            (success, latency)
        }
        Err(_) => (false, None),
    }
}
