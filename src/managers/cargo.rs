use anyhow::Result;
use std::process::{Command, ExitStatus, Stdio};

use super::PackageManager;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &str {
        "cargo"
    }
    fn exists(&self) -> bool {
        // self.version().unwrap().success()
        match Command::new(self.name())
            .arg("install-update")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }

    fn list(&self) -> Result<ExitStatus> {
        self.exec(["install-update", "--list"].into())
    }

    fn update(&self) -> Result<ExitStatus> {
        self.exec(["install-update", "--list"].into())
    }

    fn upgrade(&self) -> Result<ExitStatus> {
        self.exec(["install-update", "--all"].into())
    }

    fn install(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = std::vec!["binstall"];
        args.extend(packages);
        self.exec(args)
    }

    fn info(&self, package: &str) -> Result<ExitStatus> {
        self.exec(std::vec!["info", package])
    }

    fn search(&self, package: &str) -> Result<ExitStatus> {
        self.exec(std::vec!["search", package])
    }

    fn version(&self) -> Result<ExitStatus> {
        self.exec(["--version"].into())
    }
}
