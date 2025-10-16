# rsdish

[中文](README.md) |
[English](README_en.md)

[<img src="assets/parfait_gpt.png" width="40%" alt="Parfait logo">](#)

[![Rust](https://img.shields.io/badge/rust-1.73+-orange.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

针对家用存储的多功能同步工具，需搭配rclone食用~

## 亮点
- ⚡️ 只需一行命令即可生成rclone同步脚本；
- 🛡️ 针对可能离线的家用硬盘设计；
- 🔗 进阶：存储库符号链接支持；
- 🖥️ 跨平台：Linux, Windows, MacOS支持；

## 安装

将`rsdish`和`rclone`均添加到`PATH`；或者在`rsdish.config.toml`中配置`rclone`可执行文件路径。

## 原理
![how_it_works](assets/how_it_works.png)

## 配置方法

`rsdish.config.toml`: (如果rclone_path为空，rsdish默认会尝试直接运行环境中的rclone)

```toml
rclone_path = "<YOUR_RCLONE_PATH>"
custom_storages = ["<STG_ABS_PATH>",...]
```

`rsdish.cabinet.toml`: (运行`rsdish cabinet init`会自动生成，运行`rsdish cabinet join`会生成一个随机membership)

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

## 注意事项

⚠️ Windows平台下，`rsdish link`需要管理员权限，或者在Win10中开启开发者模式才能正常运行。