use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils;

const ERROR_PREDICATE: &str = "Config creation";

pub trait StorageContent: Serialize + for<'a> Deserialize<'a> {}

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        let path;

        if cfg!(debug_assertions) {
            path = PathBuf::from("./storage.json");

            fs::write(&path, "").map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))?;
        } else {
            let config_dir = dirs::config_dir();

            match config_dir {
                None => return Err(String::from("Failed to create config file")),
                Some(mut config_dir) => {
                    config_dir.push("./blacksmith/storage.json");

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

    pub fn save<T: StorageContent>(&self, content: T) -> Result<(), String> {
        let json = serde_json::to_string(&content)
            .map_err(|e| utils::error_to_string(e, "Saving config"))?;

        fs::write(&self.path, json).map_err(|e| utils::error_to_string(e, "Saving config"))?;

        Ok(())
    }

    pub fn load<T: StorageContent>(&self) -> Result<T, String> {
        let json = fs::read_to_string(&self.path)
            .map_err(|e| utils::error_to_string(e, "Loading config"))?;

        let value = serde_json::from_str::<T>(&json)
            .map_err(|e| utils::error_to_string(e, "Loading config"))?;

        Ok(value)
    }

    pub fn clear(&self) -> Result<(), String> {
        fs::remove_file(&self.path).map_err(|e| utils::error_to_string(e, "Clearing config"))?;

        Ok(())
    }
}
