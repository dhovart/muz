use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tokio::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub library_path: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            library_path: PathBuf::from("/System/Library/Sounds"),
        }
    }
}

impl AppConfig {
    pub async fn load(app_handle: &AppHandle) -> Result<Self> {
        let config_path = Self::get_config_path(app_handle)?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config: AppConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let default_config = AppConfig::default();
            default_config.save(app_handle).await?;
            Ok(default_config)
        }
    }

    pub async fn save(&self, app_handle: &AppHandle) -> Result<()> {
        let config_path = Self::get_config_path(app_handle)?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content).await?;
        Ok(())
    }

    fn get_config_path(app_handle: &AppHandle) -> Result<PathBuf> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| anyhow::anyhow!("Failed to get app data dir: {}", e))?;
        Ok(app_data_dir.join("config.json"))
    }

    pub fn update_library_path(&mut self, path: PathBuf) -> Result<()> {
        if !path.exists() {
            return Err(anyhow::anyhow!("Library path does not exist: {:?}", path));
        }
        if !path.is_dir() {
            return Err(anyhow::anyhow!(
                "Library path is not a directory: {:?}",
                path
            ));
        }
        self.library_path = path;
        Ok(())
    }
}
