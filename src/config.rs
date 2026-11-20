use std::path::{Path, PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub format: String,
    pub thetvdb_key: Option<String>,
    pub anidb_key: Option<String>,
}

const ENV_FILENAME: &str = ".env";
const DEFAULT_FORMAT: &str = "{n} - {s00e00} - {t}";

pub fn user_env_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "moniker").map(|dirs| dirs.config_dir().join(ENV_FILENAME))
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: String::from(DEFAULT_FORMAT),
            thetvdb_key: None,
            anidb_key: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Self::default();
        if let Some(path) = user_env_path() {
            if !path.exists() {
                Self::write_defaults(&path)?;
            }
            dotenvy::from_filename(path)?;
            config = envy::from_env::<Config>()?;
        }
        Ok(config)
    }

    fn write_defaults(path: &Path) -> Result<()> {
        let config = Self::default();
        if let Some(path) = path.parent() {
            std::fs::create_dir_all(path)?;
        }
        serde_envfile::to_file(path, &config)?;
        Ok(())
    }

    pub fn format(&self) -> &String {
        &self.format
    }
}
