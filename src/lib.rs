use std::{
    fmt::{Debug, Display},
    process::{Command, ExitStatus, Stdio},
};

use anyhow::{Context, Result};

pub trait PackageManager {
    fn name(&self) -> &str;
    fn exists(&self) -> bool {
        match Command::new(self.name())
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
        self.exec(["list"].into())
    }
    fn update(&self) -> Result<ExitStatus> {
        self.exec(["update"].into())
    }
    fn upgrade(&self) -> Result<ExitStatus> {
        self.exec(["upgrade"].into())
    }
    fn install(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["install"];
        args.extend(packages);
        self.exec(args)
    }
    fn uninstall(&self, packages: Vec<&str>) -> Result<ExitStatus> {
        let mut args = vec!["uninstall"];
        args.extend(packages);
        self.exec(args)
    }
    fn info(&self, package: &str) -> Result<ExitStatus> {
        self.exec(vec!["info", package])
    }
    fn search(&self, package: &str) -> Result<ExitStatus> {
        self.exec(vec!["search", package])
    }
    fn version(&self) -> Result<ExitStatus> {
        self.exec(["--version"].into())
    }
    fn exec(&self, args: Vec<&str>) -> Result<ExitStatus> {
        let status = Command::new(self.name())
            .args(args)
            .status()
            .with_context(|| format!("Error running command '{}'", self.name()))?;
        Ok(status)
    }
}

impl Debug for dyn PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // self.name().fmt(f)
        write!(f, "PackageManager({})", self.name())
    }
}

impl Display for dyn PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
