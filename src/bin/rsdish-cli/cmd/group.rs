use std::{collections::BTreeMap};

use clap::{Args, Subcommand};
use rsdish::logi::{
    gp::{Group, build_group_map_from_storages},
    vmem::build_virtual_member_from_group,
};
use tracing::{error, info};

use crate::cmd::storage::{storages};

#[derive(Debug, Args)]
#[command(about = "Group operations.")]
pub struct GroupCmd {
    #[command(subcommand)]
    pub subcmd: GroupSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum GroupSubcommand {
    List(GroupListArgs),
    Sync(GroupSyncArgs),
    Link(GroupLinkArgs),
    Exec(GroupExecArgs),
}

#[derive(Debug, Args)]
#[command(about = "List scanned groups.")]
pub struct GroupListArgs {
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
#[command(about = "Sync given groups.")]
pub struct GroupSyncArgs {
    #[arg(long)]
    pub strict: bool,
    #[arg(long)]
    pub all: bool,
    #[arg(value_name = "group_uuid(s)")]
    pub group_uuids: Vec<String>,
}

#[derive(Debug, Args)]
#[command(about = "Link given groups.")]
pub struct GroupLinkArgs {
    #[arg(long)]
    pub all: bool,
    #[arg(value_name = "group_uuid(s)")]
    pub group_uuids: Vec<String>,
}

#[derive(Debug, Args)]
#[command(about = "Exec script under each member root.")]
pub struct GroupExecArgs {
    #[arg(short, long, value_name = "\'SCRIPT\'")]
    pub script: String,
    #[arg(long)]
    pub all: bool,
    #[arg(value_name = "group_uuid(s)")]
    pub group_uuids: Vec<String>,
}

pub fn handle_group(cmd: GroupCmd) {
    match cmd.subcmd {
        GroupSubcommand::List(child) => handle_group_list(child),
        GroupSubcommand::Sync(child) => handle_group_sync(child),
        GroupSubcommand::Link(child) => handle_group_link(child),
        GroupSubcommand::Exec(child) => handle_group_exec(child),
    }
}

pub fn handle_group_list(args: GroupListArgs) {
    let stgs = storages();
    let gp_map = build_group_map_from_storages(&stgs);
    let gps: Vec<Group> = gp_map.into_values().collect();

    if !args.verbose {
        for gp in &gps {
            println!("Group: {:?}", gp.gp_info.gp_uuid);
            let mem_count = gp.mems.len();
            for (i, mem) in gp.mems.iter().enumerate() {
                let prefix = if i == mem_count - 1 {
                    "└──"
                } else {
                    "├──"
                };
                println!(
                    "{} Member: {:?}",
                    prefix,
                    mem.mem_info.cab_info.abs_path
                );
            }
        }
        return;
    }

    for gp in gps {
        println!("{:#?}", gp);
    }
}

fn select_groups<'a>(gp_map: &'a BTreeMap<String, Group>, all: bool, select_uuids: &Vec<String>) -> Vec<&'a Group> {
    let select_gps: Vec<&Group> = if !all {
        select_uuids
            .iter()
            .filter_map(|uuid| match gp_map.get(uuid) {
                Some(gp) => Some(gp),
                None => {
                    error!("Invalid given uuid {}", uuid);
                    None
                }
            })
            .collect()
    } else {
        gp_map.values().collect()
    };

    select_gps
}

pub fn handle_group_sync(args: GroupSyncArgs) {
    let stgs = storages();
    let gp_map = build_group_map_from_storages(&stgs);
    let select_gps: Vec<&Group> = select_groups(&gp_map, args.all, &args.group_uuids);

    let sync_target_uuids: Vec<_> = select_gps.iter().map(|g| &g.gp_info.gp_uuid).collect();
    info!("run sync for groups {:?}", sync_target_uuids);

    for select_gp in select_gps {
        let vmem = build_virtual_member_from_group(select_gp);
        select_gp.sync_from_vmem(&vmem, args.strict);
    }
}

pub fn handle_group_link(args: GroupLinkArgs) {
    let stgs = storages();
    let gp_map = build_group_map_from_storages(&stgs);
    let select_gps: Vec<&Group> = select_groups(&gp_map, args.all, &args.group_uuids);

    let link_target_uuids: Vec<_> = select_gps.iter().map(|g| &g.gp_info.gp_uuid).collect();
    info!("run link for groups {:?}", link_target_uuids);

    for select_gp in select_gps {
        let vmem = build_virtual_member_from_group(select_gp);
        select_gp.link_from_vmem(&vmem);
    }
}

pub fn handle_group_exec(args: GroupExecArgs) {
    let stgs = storages();
    let gp_map = build_group_map_from_storages(&stgs);
    let select_gps: Vec<&Group> = select_groups(&gp_map, args.all, &args.group_uuids);

    select_gps.iter().for_each(|g| g.exec(&args.script));
}
