use std::path::PathBuf;

use clap::{Args, Subcommand};
use rsdish::{
    phy::{
        dk::disks,
        stg::{Storage, build_storages_from_paths},
    },
    user::user_conf::user_conf,
};

#[derive(Debug, Args)]
#[command(about = "Info of storages.")]
pub struct StorageCmd {
    #[command(subcommand)]
    pub subcmd: StorageSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum StorageSubcommand {
    List(StorageListArgs),
}

#[derive(Debug, Args)]
#[command(about = "List all scanned storages.")]
pub struct StorageListArgs {
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn handle_storage(cmd: StorageCmd) {
    match cmd.subcmd {
        StorageSubcommand::List(child) => handle_storage_list(child),
    }
}

pub fn storages() -> Vec<Storage> {
    let dks = disks();
    let custom_stg_paths = user_conf().custom_storages;

    let mut stg_paths = dks;
    stg_paths.extend(custom_stg_paths.iter().map(|s| PathBuf::from(s)));

    let stgs = build_storages_from_paths(&stg_paths.iter().map(|s| s.as_path()).collect());
    stgs
}

pub fn handle_storage_list(args: StorageListArgs) {
    let stgs = storages();

    if !args.verbose {
        // TODO: print like storage tree, storage only print storage_abs_path, cabinet only print groupuuids
        for stg in stgs.iter() {
            println!("Storage: \"{}\"", stg.stg_info.abs_path.display());

            let cabs = &stg.cabs;
            for (j, cab) in cabs.iter().enumerate() {
                let cab_prefix = if j == cabs.len() - 1 { "└──" } else { "├──" };
                let cab_rel_path = cab.cab_info.abs_path.strip_prefix(stg.stg_info.abs_path.as_path()).unwrap();
                let group_uuids: Vec<String> = cab
                    .cab_info
                    .cab_conf
                    .memberships
                    .iter()
                    .map(|m| m.group_uuid.clone())
                    .collect();
                println!("{} Cabinet: \"{}\", memberships: {:?}", cab_prefix, cab_rel_path.display(), group_uuids);
            }
        }
        return;
    }

    for stg in stgs {
        println!("{:#?}", stg);
    }
}
