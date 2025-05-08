use std::path::PathBuf;

use config::{Config, File};
use serde::{Deserialize, Serialize};
use tauri::{fs, Manager};
use thiserror::Error;
use tokio::fs::{create_dir_all, write};
use tracing::debug;

use crate::clients::openai::OpenAiConfig;
// TODO: Write this for tauri -> sandboxing!

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    openai_config: OpenAiConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            openai_config: OpenAiConfig::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    IoError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Error while building config: {0}")]
    ConfigBuildingError(String),
    #[error("Unhandled error: {0}")]
    UnhandledError(String),
}

const CONFIG_FILE_NAME: &str = "configuration.toml";

// TODO: Allow for saved configs somewhere in env
pub async fn load(app: &tauri::AppHandle) -> Result<AppConfig, ConfigError> {
    let config_dir = ensure_config_dir(app).await?;
    let config_path = config_dir.join(format!("/{0}", CONFIG_FILE_NAME));

    // create config dir if it does not exist

    if !config_path.exists() {
        debug!(
            "Config file does not exist on path '{:?}', creating",
            &config_path
        );
        let default_config = AppConfig::default();
        let toml_content = toml::to_string(&default_config)
            .map_err(|err| ConfigError::SerializationError(err.to_string()))?;

        debug!("Writing default config file or updating missing default values.");
        write(&config_path, toml_content).await.map_err(|err| {
            ConfigError::IoError(format!(
                "Error writing config file: {:?}",
                err.raw_os_error()
            ))
            .into()
        })?;
    }

    let cfg = Config::builder()
        .add_source(File::from(config_path).format(config::FileFormat::Toml))
        .build()
        .map_err(|err| ConfigError::ConfigBuildingError(err.to_string()))?
        .try_deserialize()
        .map_err(|err| ConfigError::SerializationError(err.to_string()))?;

    Ok(cfg)
}

pub async fn save(app: &tauri::AppHandle, config: AppConfig) -> Result<(), ConfigError> {
    // TODO: Provide save function
    Ok(())
}

async fn ensure_config_dir(app: &tauri::AppHandle) -> Result<PathBuf, ConfigError> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|err| ConfigError::UnhandledError(err.to_string()))?;

    create_dir_all(&config_dir)
        .await
        .map_err(|err| ConfigError::IoError(err.to_string()))?;

    Ok(config_dir)
}

#[cfg(test)]
mod tests {
    use mockall::automock;

    use super::*;

    #[automock]
    trait ReadFile {
        fn read_to_string(&mut self, buf: &mut String) -> Result<usize, ConfigError>;
    }

    struct MockFile {
        content: String,
    }

    #[test]
    fn test_load() {
        // TODO!
    }
}
