use std::fs;
use std::path::Path;

use crate::config::{Config, CONFIG_FILE, RINGSIDE_DIR};
use crate::error::{Result, RingsideError};

pub fn run(root: Option<&str>) -> Result<()> {
    let dir_path = Path::new(RINGSIDE_DIR);
    let config_path = Path::new(CONFIG_FILE);

    if config_path.exists() {
        return Err(RingsideError::ConfigAlreadyExists(CONFIG_FILE.to_string()));
    }

    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    let config = Config {
        root: root.unwrap_or(".agents").to_string(),
        ..Default::default()
    };
    fs::write(config_path, config.to_toml()?)?;

    println!("Created {CONFIG_FILE}");
    Ok(())
}
