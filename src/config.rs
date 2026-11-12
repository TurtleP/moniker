use std::path::PathBuf;
use std::{fs, path::Path};

use anyhow::{Result, bail};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub general: General,
    pub api_keys: ApiKeys,
}

const DEFAULT_FILES: &[&str; 3] = &["mkv", "mp4", "avi"];

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General {
                files: DEFAULT_FILES.map(String::from).to_vec(),
                format: "{n} - {s00e00} - {t}".to_string(),
            },
            api_keys: ApiKeys::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct General {
    pub files: Vec<String>,
    pub format: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ApiKeys {
    pub thetvdb: String,
    pub anidb: String,
}

const CONFIG_FILE: &str = "config.toml";

fn config_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "moniker").map(|dirs| dirs.config_dir().join(CONFIG_FILE))
}

impl Config {
    pub fn load() -> Result<Self> {
        let Some(path) = config_path() else {
            return Ok(Self::default());
        };
        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn format(&self) -> &str {
        &self.general.format
    }

    pub fn save(&self) -> Result<()> {
        let Some(path) = config_path() else {
            return Ok(());
        };
        match path.parent() {
            Some(parent) => fs::create_dir_all(parent)?,
            None => bail!("config path has no parent directory"),
        }
        fs::write(path, toml::to_string(self)?)?;
        Ok(())
    }
}
