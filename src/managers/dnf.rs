use anyhow::Result;
use std::process::ExitStatus;

use super::PackageManager;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &str {
        "dnf"
    }

    fn list(&self) -> Result<ExitStatus> {
        self.exec(["list", "--installed"].into())
    }

    fn uninstall(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["remove"];
        args.extend(packages);
        self.exec(args)
    }

    fn update(&self) -> Result<ExitStatus> {
        self.exec(["check-upgrade"].into())
    }
}
