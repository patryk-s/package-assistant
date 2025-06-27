use anyhow::Result;
use std::process::ExitStatus;

use super::PackageManager;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &'static str {
        "flatpak"
    }

    fn upgrade(&self) -> Result<ExitStatus> {
        self.exec(["update"].into())
    }
}
