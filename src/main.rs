use anyhow::{Context, Result};
use std::process::ExitStatus;

mod cli;
mod config;
mod managers;

use cli::{Cli, Command, Parser};

fn main() -> Result<()> {
    let args = Cli::parse();
    let cfg =
        config::get().context("Problem reading config file. Run 'pa config' to update it.")?;

    let status = match args.command {
        Command::Config => {
            config::init().context("Saving config file")?;
            ExitStatus::default()
        }
        Command::List => managers::list(&cfg, &args)?,
        Command::Version => managers::version(&cfg, &args)?,
        Command::Managers => {
            cfg.list();
            ExitStatus::default()
        }
        Command::Info(ref p) => managers::info(&cfg, &args, &p.package)?,
        Command::Search(ref p) => managers::search(&cfg, &args, &p.package)?,
        Command::Update => managers::update(&cfg, &args)?,
        Command::Upgrade => managers::upgrade(&cfg, &args)?,
        Command::Install { ref packages } => managers::install(&cfg, &args, packages)?,
        Command::Uninstall { ref packages } => managers::uninstall(&cfg, &args, packages)?,
    };

    match status.code() {
        Some(code) => std::process::exit(code),
        None => println!("Process terminated by signal"),
    }
    Ok(())
}
