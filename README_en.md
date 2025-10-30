# rsdish

[简体中文](README.md) |
[English](README_en.md)

[<img src="assets/parfait_gpt.png" width="25%" alt="Parfait logo">](#)

[![Rust](https://img.shields.io/badge/rust-1.73+-orange.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

A multi-functional sync tool for domestic storages.

## Highlights
- ✅ Backup private data once — keep it synced forever;
- 🛡️ Designed for home drives that may go offline;
- 🔗 Unify scattered data across multiple storage devices via symbolic links;
- 🖥️ Supports Linux, Windows, and macOS;

## Installation

Add `rsdish` to `PATH`;

## How it works
[<img src="assets/how_it_works.png" width="40%" alt="How_it_works">](#)

## Configuration

```toml
# rsdish.config.toml

# macOS: ~/Library/Application Support/<app>/<config_name>.toml
# Linux: ~/.config/<app>/<config_name>.toml
# Windows: %APPDATA%\<app>\<config_name>.toml

# Tip: Run `rsdish config` to print current config path

custom_storages = ["<STG_ABS_PATH>(s)"]
```

```toml
# rsdish.cabinet.toml

# For example:
# Storage_SSD/
# ├── Cabinet_Book/
# │   ├── book1.epub
# │   ├── book2.pdf
# │   ├── .srcignore
# │   └── rsdish.cabinet.toml
# └── Cabinet_Movie/
#     ├── movie1.mp4
#     └── rsdish.cabinet.toml

# Tip: Run `rsdish cabinet init` to generate an empty config file;
# Run `rsdish cabinet join` to generate a random membership.

[[memberships]]
group_uuid = "0199ebad-44ad-78a2-baad-c56a052e33ac"
priority = 0   # Higher number = higher priority (higher can override lower)

[memberships.src_option]
enable = false

[memberships.dst_option]
enable = false
cover_level = 0  # Enum: 0=DontCover, 1=HigherCover
save_level  = 0  # Enum: 0=DontSave, 1=SaveHigher, 2=SaveHigherEqual, 3=SaveAll

[memberships.link_option]
enable = false
save_level = 0
```

```ignore
# .srcignore
# The syntax of .srcignore is largely the same as that of .gitignore.
```
## NOTE

⚠️ On Windows, `rsdish link` must be run with administrator privileges, or Developer Mode must be enabled on Windows 10 for proper operation.

## License

This project is licensed under the [GNU General Public License v3.0 (GPLv3)](LICENSE).
