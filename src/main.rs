use std::path::{Path, PathBuf};

mod app;
use app::App;

mod state;
mod files;
mod keys;
mod ui;

mod config;
use config::Config;

mod logging;
mod matchers;
mod _ui;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let _guard = logging::init_logging(Path::new("logs"));

    let cli = Cli::parse();
    let config = Config::load()?;
    let mut app = App::new(cli.dir, config)?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    ratatui::run(|terminal| runtime.block_on(app.run(terminal)))
}
