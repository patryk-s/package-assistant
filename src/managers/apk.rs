use anyhow::Result;
use std::process::ExitStatus;

use super::PackageManager;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &'static str {
        "apk"
    }

    fn list(&self) -> Result<ExitStatus> {
        self.exec(["list", "--installed"].into())
    }

    fn install(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = std::vec!["add"];
        args.extend(packages);
        self.exec(args)
    }

    fn uninstall(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["del"];
        args.extend(packages);
        self.exec(args)
    }
}
