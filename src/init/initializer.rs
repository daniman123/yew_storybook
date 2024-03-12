use crate::prelude::*;
use std::{fs, path::Path};

/// Struct representing project paths.
#[derive(Debug)]
pub struct ProjectPaths {
    /// Path to the directory containing test components.
    pub test_components_path: String,
    /// Indicates whether the config path exists.
    pub config_path: String,
}

impl Default for ProjectPaths {
    fn default() -> Self {
        Self {
            test_components_path: r"./src/components".into(),
            config_path: r"./src/yew_storybook".into(),
        }
    }
}

impl ProjectPaths {
    /// Creates a new instance of ProjectPaths with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the project paths.
    pub fn initialize_project() -> Result<Self> {
        let mut project_paths = ProjectPaths::new();

        if !Path::new(&project_paths.test_components_path).exists() {
            project_paths.test_components_path = String::from("./src");
        }

        if !Path::new(&project_paths.config_path).exists() {
            create_yew_storybook_directory()?;
        }

        project_paths.config_path = format!("{}/{}", project_paths.config_path, "yew_config.json");

        Ok(project_paths)
    }
}

/// Creates the "src/yew_storybook" directory if it does not exist.
fn create_yew_storybook_directory() -> Result<()> {
    Ok(fs::create_dir("src/yew_storybook").map_err(|e| {
        eprintln!("Failed to create src/yew_storybook directory: {}", e);
        e
    })?)
}
