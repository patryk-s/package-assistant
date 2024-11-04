use anyhow::Result;
use package_assistant::PackageManager;
use std::process::ExitStatus;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &str {
        "apt"
    }

    fn list(&self) -> Result<ExitStatus> {
        self.exec(["list", "--installed"].into())
    }
}
