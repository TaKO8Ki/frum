<div align="center">

# farm

A blazing fast and simple Ruby version manager written in Rust

[![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/farm/CI/main)](https://github.com/TaKO8Ki/farm/actions)

[Usage](##Usage) | [Docs](#)

</div>

## Usage

### Subcommands

- install-list: Lists the Ruby versions available to install.
- install: Installs `[VERSION]`. If no version provided, it will install the version specified in the `.ruby-version` files located in the current working directory.
