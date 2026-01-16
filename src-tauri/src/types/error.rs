use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("File error: {0}")]
    File(#[from] io::Error),

    #[error("Internal error: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid path: {0}")]
    PathError(String),
}

impl serde::Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug)]
pub enum ValidationError {
    MissingWinePrefix,
    MissingGameLauncher,
    // MissingWineBinDir,
    MissingDXVKSymlinks,
    // MissingVCRuntimes,
}
