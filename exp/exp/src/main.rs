mod app;
mod config;
mod http;
mod logging;
mod models;
mod ping;
mod ui;

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::fs;
use std::io;

use crate::app::App;
use crate::config::Config;
use crate::ui::run_app;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Get the configuration file path from the command line arguments
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    // Load configuration
    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", config_path));
    let config: Config = toml::from_str(&config_content)
        .unwrap_or_else(|_| panic!("Failed to parse {}", config_path));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let app = App::new(config.targets);
    let res = run_app(app, &mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("Error: {}", err);
    }

    Ok(())
}
