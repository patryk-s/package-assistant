use std::path::PathBuf;

use confy::ConfyError;
use directories::ProjectDirs;
use inquire::{MultiSelect, Select};
use serde::{Deserialize, Serialize};

use crate::managers::discover_managers;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PaConfig {
    pub(crate) default_manager: String,
    pub(crate) managers: Vec<String>,
}
impl PaConfig {
    pub(crate) fn list(&self) -> Result<(), anyhow::Error> {
        for manager in &self.managers {
            println!(" * {manager}");
        }
        println!("Main manager: {}", self.default_manager);
        Ok(())
    }
}

impl Default for PaConfig {
    fn default() -> Self {
        let managers = MultiSelect::new("Select package managers", discover_managers())
            .prompt()
            .expect("multiselect");
        let default_manager = Select::new("Choose main package manager", managers.clone())
            .prompt()
            .expect("default manager")
            .to_string();
        Self {
            default_manager,
            managers,
        }
    }
}

pub(crate) fn get() -> Result<PaConfig, ConfyError> {
    let path = config_path();
    confy::load_path(path)
}

pub(crate) fn init() -> Result<(), ConfyError> {
    let path = config_path();
    let cfg = PaConfig::default();
    confy::store_path(path, cfg)
}

fn config_path() -> PathBuf {
    ProjectDirs::from("", "", "pa")
        .expect("project dir missing")
        .config_dir()
        .to_path_buf()
        .join("config.toml")
}
