#[cfg(not(target_os = "macos"))]
mod apk;

#[cfg(not(target_os = "macos"))]
mod apt;

#[cfg(not(target_os = "macos"))]
mod snap;

#[cfg(not(target_os = "macos"))]
mod dnf;

#[cfg(not(target_os = "macos"))]
mod pkg;

#[cfg(not(target_os = "macos"))]
mod pacman;

#[cfg(not(target_os = "macos"))]
mod paru;

#[cfg(not(target_os = "macos"))]
mod yay;

#[cfg(not(target_os = "macos"))]
mod flatpak;

mod cargo;
mod homebrew;
mod nix_env;

use anyhow::{bail, Context, Result};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    process::{Command, ExitStatus, Stdio},
};

use crate::{config::PaConfig, Cli};

pub(crate) type Managers = HashMap<String, Box<dyn PackageManager>>;

pub(crate) fn discover_managers() -> Vec<String> {
    let mut managers = Vec::new();
    // TODO: do actual command discovery
    let all_managers = all_managers();
    for (manager, _) in all_managers {
        managers.push(manager);
    }
    managers
}

// TODO: create a macro_rules to generate this function
#[cfg(not(target_os = "macos"))]
/// Returns all package managers (for current platform)
fn all_managers() -> Managers {
    let manager_brew = homebrew::Manager;
    let manager_cargo = cargo::Manager;
    let manager_apk = apk::Manager;
    let manager_apt = apt::Manager;
    let manager_pkg = pkg::Manager;
    let manager_snap = snap::Manager;
    let manager_dnf = dnf::Manager;
    let manager_nix_env = nix_env::Manager;
    let manager_pacman = pacman::Manager;
    let manager_paru = paru::Manager;
    let manager_yay = yay::Manager;
    let manager_flatpak = flatpak::Manager;
    let all_managers: Vec<Box<dyn PackageManager>> = vec![
        Box::new(manager_brew),
        Box::new(manager_cargo),
        Box::new(manager_apk),
        Box::new(manager_apt),
        Box::new(manager_pkg),
        Box::new(manager_snap),
        Box::new(manager_dnf),
        Box::new(manager_nix_env),
        Box::new(manager_pacman),
        Box::new(manager_paru),
        Box::new(manager_yay),
        Box::new(manager_flatpak),
    ];
    let mut managers = Managers::new();
    for manager in all_managers {
        if manager.exists() {
            managers.insert(manager.name().to_string(), manager);
        }
    }
    managers
}

#[cfg(target_os = "macos")]
/// Returns all package managers (for current platform)
fn all_managers() -> Managers {
    let manager_brew = homebrew::Manager;
    let manager_cargo = cargo::Manager;
    let manager_nix_env = nix_env::Manager;
    let all_managers: Vec<Box<dyn PackageManager>> = vec![
        Box::new(manager_brew),
        Box::new(manager_cargo),
        Box::new(manager_nix_env),
    ];
    let mut managers = Managers::new();
    for manager in all_managers {
        if manager.exists() {
            managers.insert(manager.name().to_string(), manager);
        }
    }
    managers
}

fn get_managers(cfg: &PaConfig, cli: &Cli) -> Result<Managers> {
    let managers: Managers = all_managers()
        .into_iter()
        .filter(|(name, _manager)| {
            if cli.all_managers {
                cfg.managers.contains(name)
            } else if cli.manager.is_some() {
                name == cli.manager.as_ref().unwrap()
            } else {
                name == &cfg.default_manager
            }
        })
        .collect();
    if managers.is_empty() {
        bail!("Package manager not found");
    }
    Ok(managers)
}

pub(crate) fn list(cfg: &PaConfig, cli: &Cli) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.list()
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) fn update(cfg: &PaConfig, cli: &Cli) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.update()
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) fn upgrade(cfg: &PaConfig, cli: &Cli) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.upgrade()
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) fn install(cfg: &PaConfig, cli: &Cli, packages: &[String]) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.install(packages.iter().map(AsRef::as_ref).collect())
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) fn uninstall(cfg: &PaConfig, cli: &Cli, packages: &[String]) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.uninstall(packages.iter().map(AsRef::as_ref).collect())
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) fn version(cfg: &PaConfig, cli: &Cli) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.version()
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) fn info(cfg: &PaConfig, cli: &Cli, package: &str) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.info(package)
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) fn search(cfg: &PaConfig, cli: &Cli, package: &str) -> Result<ExitStatus> {
    let managers = get_managers(cfg, cli)?;
    let _results: Result<Vec<_>, _> = managers
        .values()
        .map(|m| {
            println!("## {}", m.name());
            m.search(package)
        })
        .collect();
    Ok(ExitStatus::default())
}

pub(crate) trait PackageManager {
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
