use std::fs;
use std::path::Path;

use crate::config::{Config, Source, CONFIG_FILE};
use crate::error::{Result, RingsideError};
use crate::source::parse_source;

pub fn run(url: &str, dest: Option<&str>) -> Result<()> {
    let parsed = parse_source(url);
    let mut config = Config::load_or_create()?;

    let source = Source {
        url: parsed.url.clone(),
        path: parsed.path,
        dest: dest.map(std::string::ToString::to_string),
        git_ref: parsed.git_ref,
    };

    if config
        .sources
        .iter()
        .any(|s| s.url == source.url && s.dest == source.dest)
    {
        return Err(RingsideError::ConfigParse(
            "Source already exists in config".to_string(),
        ));
    }

    config.sources.push(source);
    fs::write(Path::new(CONFIG_FILE), config.to_toml()?)?;

    println!("Added {} -> {}", parsed.url, dest.unwrap_or("/"));
    Ok(())
}
