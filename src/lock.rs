use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::config::RINGSIDE_DIR;
use crate::error::Result;

const LOCK_FILE: &str = ".ringside/lock.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LockFile {
    #[serde(default)]
    pub sources: HashMap<String, LockedSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedSource {
    pub url: String,
    pub commit: String,
    pub synced_at: String,
}

impl LockFile {
    pub fn load() -> Result<Self> {
        let path = Path::new(LOCK_FILE);
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content).unwrap_or_default())
    }

    pub fn save(&self) -> Result<()> {
        let dir = Path::new(RINGSIDE_DIR);
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(LOCK_FILE, content)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&LockedSource> {
        self.sources.get(key)
    }

    pub fn set(&mut self, key: String, source: LockedSource) {
        self.sources.insert(key, source);
    }
}

#[must_use]
pub fn lock_key(url: &str, dest: Option<&str>) -> String {
    match dest {
        Some(d) => format!("{url}:{d}"),
        None => url.to_string(),
    }
}
