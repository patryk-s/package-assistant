use anyhow::Result;
use package_assistant::PackageManager;
use std::process::ExitStatus;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &str {
        "flatpak"
    }

    fn upgrade(&self) -> Result<ExitStatus> {
        self.exec(["update"].into())
    }
}
