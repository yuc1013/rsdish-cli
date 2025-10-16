use std::{env, fs, path::PathBuf};

use clap::{Args, Subcommand};
use rsdish::phy::cab_conf::{default_cabinet_config, default_membership, CabinetConfig};

#[derive(Debug, Args)]
#[command(about = "Initialization of cabinet, membership management.")]
pub struct CabinetCmd {
    #[command(subcommand)]
    pub subcmd: CabinetSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CabinetSubcommand {
    Init(CabinetInitArgs),
    Join(CabinetJoinArgs),
}

#[derive(Debug, Args)]
#[command(about = "Init an empty config in current cabinet.")]
pub struct CabinetInitArgs {}

#[derive(Debug, Args)]
#[command(about = "Add current cabinet to existing group(s), or no args to a new group.")]
pub struct CabinetJoinArgs {
    #[arg(value_name = "uuid(s)")]
    pub group_uuids: Vec<String>,
}

pub fn handle_cabinet(cmd: CabinetCmd) {
    match cmd.subcmd {
        CabinetSubcommand::Init(child) => handle_cabinet_init(child),
        CabinetSubcommand::Join(child) => handle_cabinet_join(child),
    }
}

pub fn handle_cabinet_init(_args: CabinetInitArgs) {
    let current_dir = env::current_dir().unwrap();

    let cab_conf = default_cabinet_config();

    let toml_str = toml::to_string(&cab_conf).unwrap();

    let config_path: PathBuf = current_dir.join(env!("CABINET_CONFIG_NAME"));

    fs::write(&config_path, toml_str).unwrap();

    println!(
        "Created default cabinet config at {}",
        config_path.display()
    );
}

pub fn handle_cabinet_join(args: CabinetJoinArgs) {
    let current_dir = env::current_dir().unwrap();
    let config_path: PathBuf = current_dir.join(env!("CABINET_CONFIG_NAME"));

    let cab_conf_str = fs::read_to_string(&config_path).unwrap();
    let mut cab_conf: CabinetConfig = toml::from_str(&cab_conf_str).unwrap();

    for uuid in &args.group_uuids {
        let mut new_member = default_membership();
        new_member.group_uuid = uuid.clone();
        println!("Added new member with uuid {}", uuid);
        cab_conf.memberships.push(new_member);
    }

    // default behaviour: rsdish cabinet join will default join a random membership
    if args.group_uuids.is_empty() {
        let new_member = default_membership();
        println!("Added new member with uuid {}", new_member.group_uuid);
        cab_conf.memberships.push(new_member);
    }

    let toml_str = toml::to_string(&cab_conf).unwrap();
    fs::write(&config_path, toml_str).unwrap();

    println!("Updated cabinet config at {}", config_path.display());
}
