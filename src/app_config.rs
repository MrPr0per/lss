use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub project_path: PathBuf,
}

pub struct ConfigStorage {
    config_path: PathBuf,
}

impl ConfigStorage {
    pub fn new() -> Self {
        Self {
            config_path: Self::default_path(),
        }
    }

    fn default_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .expect("Cannot determine config directory");
        proj_dirs.config_dir().join("config.toml")
    }

    pub fn load(&self) -> Option<AppConfig> {
        if !self.config_path.exists() {
            return None;
        }
        let content = fs::read_to_string(&self.config_path).expect(&format!(
            "Failed to read config file: {}",
            self.config_path.display()
        ));
        toml::from_str(&content).expect("Failed to parse config file")
    }

    pub fn save(&self, config: &AppConfig) {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).expect(&format!(
                "Failed to create config directory: {}",
                parent.display()
            ))
        }
        let content = toml::to_string_pretty(config).expect("Failed to serialize config");
        fs::write(&self.config_path, content).expect(&format!(
            "Failed to write config file: {}",
            self.config_path.display()
        ));
    }
}
