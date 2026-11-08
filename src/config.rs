use std::fs;
use std::path::PathBuf;

use anyhow::{Result, bail};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub general: General,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General {
                files: vec!["mkv".to_string(), "mp4".to_string()],
                format: "{n} - {s00e00} - {t}".to_string(),
            },
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct General {
    pub files: Vec<String>,
    pub format: String,
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

    pub fn filter(&self, file: &str) -> bool {
        self.general.files.iter().any(|ext| file.ends_with(ext))
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
