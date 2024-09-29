use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils;

const ERROR_PREDICATE: &str = "Config creation";

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageContent {
    pub session_key: String,
    pub username: String,
}

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        let path;

        if cfg!(debug_assertions) {
            path = PathBuf::from("./storage.json");

            if !fs::exists(&path).map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))? {
                fs::write(&path, "").map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))?;
            }
        } else {
            let config_dir = dirs::config_dir();

            match config_dir {
                None => return Err(String::from("Failed to create config file")),
                Some(mut config_dir) => {
                    config_dir.push("ironworker");

                    fs::create_dir_all(&config_dir)
                        .map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))?;

                    config_dir.push("storage.json");

                    if !fs::exists(&config_dir)
                        .map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))?
                    {
                        fs::write(&config_dir, "")
                            .map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))?;
                    };

                    path = config_dir;
                }
            }
        };

        Ok(Self { path })
    }

    pub fn save(&self, content: StorageContent) -> Result<(), String> {
        let json = serde_json::to_string(&content)
            .map_err(|e| utils::error_to_string(e, "Saving config"))?;

        fs::write(&self.path, json).map_err(|e| utils::error_to_string(e, "Saving config"))?;

        Ok(())
    }

    pub fn load(&self) -> Result<StorageContent, String> {
        let json = fs::read_to_string(&self.path)
            .map_err(|e| utils::error_to_string(e, "Loading config"))?;

        let value = serde_json::from_str::<StorageContent>(&json)
            .map_err(|e| utils::error_to_string(e, "Loading config"))?;

        Ok(value)
    }

    pub fn clear(&self) -> Result<(), String> {
        fs::remove_file(&self.path).map_err(|e| utils::error_to_string(e, "Clearing config"))?;

        Ok(())
    }
}
