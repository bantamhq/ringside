use thiserror::Error;

#[derive(Error, Debug)]
pub enum RingsideError {
    #[error("Config file not found: {0}")]
    ConfigNotFound(String),

    #[error("Failed to parse config: {0}")]
    ConfigParse(String),

    #[error("Config file already exists: {0}")]
    ConfigAlreadyExists(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("Failed to create directory: {0}")]
    DirectoryCreation(String),

    #[error("Failed to copy files: {0}")]
    FileCopy(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialize(#[from] toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, RingsideError>;
