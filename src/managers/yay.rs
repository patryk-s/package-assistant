use anyhow::Result;
use std::process::ExitStatus;

use super::PackageManager;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &str {
        "yay"
    }

    fn list(&self) -> Result<ExitStatus> {
        self.exec(["--query"].into())
    }

    fn uninstall(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["--remove"];
        args.extend(packages);
        self.exec(args)
    }

    fn update(&self) -> Result<ExitStatus> {
        self.exec(["--sync", "--refresh"].into())
    }

    fn upgrade(&self) -> Result<ExitStatus> {
        self.exec(["--sync", "--sysupgrade"].into())
    }

    fn install(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["--sync"];
        args.extend(packages);
        self.exec(args)
    }

    fn info(&self, package: &str) -> Result<ExitStatus> {
        self.exec(vec!["--sync", "--info", package])
    }

    fn search(&self, package: &str) -> Result<ExitStatus> {
        self.exec(vec!["--sync", "--search", package])
    }
}
