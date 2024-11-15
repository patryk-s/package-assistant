use std::path::PathBuf;

use confy::ConfyError;
use dialoguer::{MultiSelect, Select};
use directories::ProjectDirs;
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
        let managers = discover_managers();
        if managers.is_empty() {
            eprintln!("No supported manager found!");
            Self {
                default_manager: "".to_string(),
                managers: Vec::new(),
            }
        } else if managers.len() == 1 {
            Self {
                default_manager: managers[0].clone(),
                managers,
            }
        } else {
            let choices = MultiSelect::new()
                .with_prompt("Select package managers")
                .items(&managers)
                .interact()
                .expect("multiselect");
            let managers: Vec<String> = choices.into_iter().map(|i| managers[i].clone()).collect();
            let choice = Select::new()
                .with_prompt("Choose main package manager")
                .items(&managers)
                .interact()
                .expect("default manager");
            let default_manager = managers[choice].clone();
            Self {
                default_manager,
                managers,
            }
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
