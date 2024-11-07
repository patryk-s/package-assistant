# Package Assistant

Package Assistant helps you manage your package managers.

Configure which package managers you want to use and pick a main (default) package manager (run `pa config` to choose).
You can then use `pa` to run your usual package commands, for example:

```shell
pa search foo
pa info foo
pa install foo
```

If you want to run a command with all configured package managers, just add `-a`:

```shell
pa search -a foo
pa update -a
pa upgrade -a
```

You can also run a command with a specific manager:

```shell
pa install -m snap foo
```

Run `pa help` for the full usage:

```
Usage: pa [OPTIONS] <COMMAND>

Commands:
  config     Create or update configuration
  info       Show package details
  install    Install packages
  list       List installed packages
  search     Search for package
  managers   List available package managers
  uninstall  Uninstall packages
  update     Update package database
  upgrade    Upgrade installed packages
  version    Show package manager version
  help       Print this message or the help of the given subcommand(s)

Options:
  -a, --all-managers       Apply command to all configured package managers
  -m, --manager <MANAGER>  Run command with secified package manager
  -h, --help               Print help
  -V, --version            Print version
```

## Supported package managers

- apt
- cargo (uses [cargo binstall](https://github.com/cargo-bins/cargo-binstall) and [cargo install-update](https://github.com/nabijaczleweli/cargo-update))
- homebrew
- snap

## Installation

If you have `rustc` and `cargo` installed (if not, see [rustup.rs](https://rustup.rs/))
you can clone this repo, `cd` into it, and run:

```shell
cargo install --path .
```
