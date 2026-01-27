use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::error::{Result, RingsideError};

pub const RINGSIDE_DIR: &str = ".ringside";
pub const CONFIG_FILE: &str = ".ringside/config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub root: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub url: String,
    pub path: Option<String>,
    pub dest: Option<String>,
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: ".agents".to_string(),
            sources: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Path::new(CONFIG_FILE);
        if !path.exists() {
            return Err(RingsideError::ConfigNotFound(CONFIG_FILE.to_string()));
        }

        let content = fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| RingsideError::ConfigParse(e.to_string()))
    }

    pub fn load_or_create() -> Result<Self> {
        let path = Path::new(CONFIG_FILE);
        if path.exists() {
            return Self::load();
        }

        let dir = Path::new(RINGSIDE_DIR);
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let config = Self::default();
        fs::write(path, config.to_toml()?)?;
        Ok(config)
    }

    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).map_err(|e| RingsideError::ConfigParse(e.to_string()))
    }
}
