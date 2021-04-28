<div align="center">

![frum](./resources/logo.png)

**frum is currently in beta**

🏃‍♂️ A little bit fast and modern Ruby version manager written in Rust

[![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/frum/CI/main)](https://github.com/TaKO8Ki/frum/actions) [![crates](https://img.shields.io/crates/v/frum.svg?logo=rust)](https://crates.io/crates/frum)

[Usage](#Usage) | [Docs](#)

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

## Goals

- **Blazing-Fast Ruby Installation** - built with speed in mind
- **Cross-Platform** - works on macOS, Linux and (Windows)

### Benchmark

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `rbenv` | 224003.4 ± 635.3 | 222880.7 | 224699.7 | 1.03 ± 0.00 |
| `frum` | 221892.7 ± 1268.8 | 220353.6 | 223999.6 | 1.02 ± 0.01 |
| `frum(pre-release)` | 218178.1 ± 619.9 | 217431.1 | 219347.3 | 1.00 |

For more information, please see [#16](https://github.com/TaKO8Ki/frum/pull/16).

## Usage

### Shell Setup

You need to run some shell commands before using frum. All you have to do is evaluate the output of `frum init`. Check out the following guides for the shell you use:

#### Bash

add the following to your `.bashrc`:

```bash
eval "$(frum init)"
```

#### Zsh

add the following to your `.zshrc`:

```zsh
eval "$(frum init)"
```

### Options

- --log-level: The log level of frum commands [default: info] [possible values: quiet, info, error].

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

### Homebrew

If you’re using Homebrew on macOS, install the `frum` formula.

```
$ brew install tako8ki/tap/frum
```

### Cargo

If you already have a Rust environment set up, you can use the `cargo install` command:

```
$ cargo install --version 0.1.0-beta.0 frum
```

### Arch Linux

If you’re using Arch Linux, install the `frum-bin` package using your favorite AUR helper.

```
$ yay -S frum-bin
```

## Contribution

Contributions, issues and pull requests are welcome!

## Reference

- [Schniz/fnm](https://github.com/Schniz/fnm)
