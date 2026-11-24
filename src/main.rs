use std::path::{Path, PathBuf};

mod app;

mod files;
mod keys;
mod popup;
mod state;
mod ui;

mod config;
use config::Config;

mod logging;
mod matchers;

use anyhow::Result;
use clap::Parser;

use crate::config::user_env_path;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".", help = "Directory to load files from")]
    dir: PathBuf,
    #[arg(long, help = "Open the config directory")]
    open_config: bool,
}

fn main() -> Result<()> {
    let _guard = logging::init_logging(Path::new("logs"));

    let cli = Cli::parse();

    if cli.open_config {
        opener::reveal(user_env_path().expect("Project Directory exists"))?;
        return Ok(());
    }

    let config = Config::load()?;
    let mut app = app::Application::new(cli.dir);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    ratatui::run(|terminal| runtime.block_on(app.run(terminal)))
}
