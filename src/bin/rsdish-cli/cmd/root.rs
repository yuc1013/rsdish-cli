use std::env;

use clap::{Parser, Subcommand};

use clap::builder::{styling::{AnsiColor, Effects}, Styles};
use rsdish::user::user_conf::user_conf;

use crate::cmd::cabinet::{handle_cabinet, CabinetCmd};
use crate::cmd::config::{handle_config, ConfigCmd};
use crate::cmd::group::{handle_group, GroupCmd};
use crate::cmd::preset::{PresetCmd, handle_preset};
use crate::cmd::storage::{handle_storage, StorageCmd};

// Configures Clap v3-style help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(name = "rsdish")]
#[command(about = "A multi-functional sync tool for domestic storages.", long_about = None)]
#[command(styles=STYLES)]
pub struct RootCmd {
    #[command(subcommand)]
    pub subcmd: SubcommandEnum,
}

#[derive(Subcommand, Debug)]
pub enum SubcommandEnum {
    #[command(visible_alias = "cab")]
    Cabinet(CabinetCmd),
    #[command(visible_alias = "stg")]
    Storage(StorageCmd),
    #[command(visible_alias = "gp")]
    Group(GroupCmd),
    Config(ConfigCmd),
    Preset(PresetCmd),
}

pub fn handle_root(cmd: RootCmd) {
    set_env();
    match cmd.subcmd {
        SubcommandEnum::Cabinet(child) => handle_cabinet(child),
        SubcommandEnum::Storage(child) => handle_storage(child),
        SubcommandEnum::Group(child) => handle_group(child),
        SubcommandEnum::Config(child) => handle_config(child),
        SubcommandEnum::Preset(child) => handle_preset(child),
    }
}

fn set_env() {
    let user_conf = user_conf();
    unsafe {
        env::set_var("RCLONE_PATH", user_conf.rclone_path);
    }
}