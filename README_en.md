# rsdish

[中文](README.md) |
[English](README_en.md)

[<img src="assets/parfait_gpt.png" width="40%" alt="Parfait logo">](#)

[![Rust](https://img.shields.io/badge/rust-1.73+-orange.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

A multi-functional sync tool for domestic storages, core functionality relies on rclone.

## Highlights
- ⚡️ Generate rclone scripts in single one command line.
- 🛡️ Designed for offline-able domestic disks.
- 🔗 Advanced: Support for symlink to cabinet.
- 🖥️ Cross-platform: Linux, Windows and MacOS.

## Installation

Add `rsdish` and `rclone` to `PATH`; Or configure `rclone_path` in `rsdish.config.toml`.

## How it works
![how_it_works](assets/how_it_works.png)

## Configuration

`rsdish.config.toml`: (if rclone_path is empty，rsdish will try rclone in env)

```toml
rclone_path = "<YOUR_RCLONE_PATH>"
custom_storages = ["<STG_ABS_PATH>",...]
```

`rsdish.cabinet.toml`: (run `rsdish cabinet init` to generate, run `rsdish cabinet join` to generate a random membership)

```toml
note = "New Cabinet"

[[memberships]]
group_uuid = "0199ebad-44ad-78a2-baad-c56a052e33ac"
priority = 0

[memberships.src_option]
enable = false

[memberships.dst_option]
enable = false
cover_level = 0
save_level = 0
params = ""

[memberships.link_option]
enable = false
save_level = 0
```


> Priority: Cabinet rank in a group.
>
> SaveLevel: 0-DontSave, 1-SaveHigher, 2-SaveHigherEqual, 3-SaveAll
> 
> CoverLevel: 0-DontCover, 1-HigherCover, 2-HigherEqualCover

## NOTE

⚠️ On Windows, `rsdish link` must be run with administrator privileges, or Developer Mode must be enabled on Windows 10 for proper operation.

## License

This project is licensed under the [GNU General Public License v3.0 (GPLv3)](LICENSE).