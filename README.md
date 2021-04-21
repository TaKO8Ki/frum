<div align="center">

![frum](./resources/logo.png)

🏃‍♂️ A little bit fast and modern Ruby version manager written in Rust

[![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/frum/CI/main)](https://github.com/TaKO8Ki/frum/actions)

[Usage](##Usage) | [Docs](#)

</div>

```sh
$ eval "$(frum init)"
$ frum install 2.6.5
$ frum local 2.6.5
$ ruby -v
```

## Features

- Cross-platform support (macOS, Linux)
- Works with `.ruby-version` files
- Auto-Completion

## Usage

### Subcommands

- init: Sets environment variables for initializing frum.
- install: Installs the specified Ruby version.
    - -l, --list: Lists the Ruby versions available to install.
    - -w, --with-openssl-dir: Specify the openssl directory.
- uninstall: Uninstall a specific Ruby version.
- versions: Lists installed Ruby versions.
- global: Sets the global Ruby version.
- local: Sets the current Ruby version.

## Installation

### Cargo

If you already have a Rust environment set up, you can use the `cargo install` command:

```
$ cargo install frum
```
