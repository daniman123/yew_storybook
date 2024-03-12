use crate::prelude::*;

use serde::{Deserialize, Serialize};
use std::fs;

/// Struct representing the configuration data.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    // Define your configuration fields here
    pub path_dir: String,
}

impl AppConfig {
    /// Reads the configuration data from a file.
    pub fn read_file(&self, file_path: String) -> Result<Self> {
        let content = fs::read_to_string(file_path)?;
        let config: Self = serde_json::from_str(&content).unwrap();
        Ok(config)
    }

    /// Writes the configuration data to a file.
    pub fn to_file(&self, file_path: String) -> Result<()> {
        let content = serde_json::to_string_pretty(self).unwrap();
        fs::write(file_path, content)?;
        Ok(())
    }
}
