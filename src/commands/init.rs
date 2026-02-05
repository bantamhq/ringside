use std::fs;
use std::path::Path;

use crate::config::{Config, CONFIG_FILE};
use crate::error::{Result, RingsideError};

pub fn run(root: Option<&str>) -> Result<()> {
    let config_path = Path::new(CONFIG_FILE);

    if config_path.exists() {
        return Err(RingsideError::ConfigAlreadyExists(CONFIG_FILE.to_string()));
    }

    let config = Config {
        root: root.unwrap_or(".agents").to_string(),
        ..Default::default()
    };
    fs::write(config_path, config.to_toml()?)?;

    println!("Created {CONFIG_FILE}");
    Ok(())
}
