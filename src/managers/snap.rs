use anyhow::Result;
use std::process::ExitStatus;

use super::PackageManager;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &str {
        "snap"
    }
    fn search(&self, package: &str) -> Result<ExitStatus> {
        self.exec(vec!["find", package])
    }
    fn update(&self) -> Result<ExitStatus> {
        self.exec(["refresh", "--list"].into())
    }
    fn upgrade(&self) -> Result<ExitStatus> {
        self.exec(["refresh"].into())
    }
}
