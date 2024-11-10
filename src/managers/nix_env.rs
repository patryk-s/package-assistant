use anyhow::{Context, Result};
use package_assistant::PackageManager;
use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &str {
        "nix-env"
    }

    fn list(&self) -> Result<ExitStatus> {
        self.exec(["--query", "--installed"].into())
    }

    fn uninstall(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["--uninstall"];
        args.extend(packages);
        self.exec(args)
    }

    fn update(&self) -> Result<ExitStatus> {
        println!("update not supported for {}", self.name());
        Ok(ExitStatus::from_raw(1))
    }

    fn upgrade(&self) -> Result<ExitStatus> {
        self.exec(["--upgrade"].into())
    }

    fn install(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["--install"];
        args.extend(packages);
        self.exec(args)
    }

    fn info(&self, package: &str) -> Result<ExitStatus> {
        self.exec(vec!["--query", "--description", package])
    }

    fn search(&self, package: &str) -> Result<ExitStatus> {
        self.exec(vec!["--query", "--available", "--attr-path", package])
    }

    fn exec(&self, args: Vec<&str>) -> Result<ExitStatus> {
        let status = std::process::Command::new(self.name())
            // by default, nix-env pipes its output to 'less' but has no flag to disable that
            .env("NIX_PAGER", "cat")
            .args(args)
            .status()
            .with_context(|| std::format!("Error running command '{}'", self.name()))?;
        Ok(status)
    }
}
