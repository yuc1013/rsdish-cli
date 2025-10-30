use std::path::Path;

use clap::{Args, Subcommand};
use rsdish::phy::cab::{build_cabinet_from_path, write_cabinet};

#[derive(Debug, Args)]
#[command(about = "Apply chosen presets to cabinet.")]
pub struct PresetCmd {
    #[command(subcommand)]
    pub subcmd: PresetSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PresetSubcommand {
    Main(PresetMainArgs),
    Mirror(PresetMirrorArgs),
    Gate(PresetGateArgs),
}

#[derive(Debug, Args)]
#[command(about = "priority-3, src-True, dst-True, link-False.")]
pub struct PresetMainArgs {
    #[arg(value_name = "cabinet_path(s)")]
    pub cabinet_paths: Vec<String>,
}

#[derive(Debug, Args)]
#[command(about = "priority-1, src-True, dst-True, link-False.")]
pub struct PresetMirrorArgs {
    #[arg(value_name = "cabinet_path(s)")]
    pub cabinet_paths: Vec<String>,
}

#[derive(Debug, Args)]
#[command(about = "priority-0, src-False, dst-False, link-True-SaveAll.")]
pub struct PresetGateArgs {
    #[arg(value_name = "cabinet_path(s)")]
    pub cabinet_paths: Vec<String>,
}


pub fn handle_preset(cmd: PresetCmd) {
    match cmd.subcmd {
        PresetSubcommand::Gate(child) => handle_preset_gate(child),
        PresetSubcommand::Main(child) => handle_preset_main(child),
        PresetSubcommand::Mirror(child) => handle_preset_mirror(child),
    }
}

fn handle_preset_gate(cmd: PresetGateArgs) {
    for cab_path in cmd.cabinet_paths {
        let mut cab = match build_cabinet_from_path(Path::new(&cab_path)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to build cabinet from path {}: {}", cab_path, e);
                continue;
            }
        };
        cab.cab_info.cab_conf.to_gate();
        match write_cabinet(&cab) {
            Ok(_) => println!("Applied Main preset to cabinet: {}", cab_path),
            Err(e) => eprintln!("Failed to save cabinet config for {}: {}", cab_path, e),
        }
    }
}

fn handle_preset_main(cmd: PresetMainArgs) {
    for cab_path in cmd.cabinet_paths {
        let mut cab = match build_cabinet_from_path(Path::new(&cab_path)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to build cabinet from path {}: {}", cab_path, e);
                continue;
            }
        };
        cab.cab_info.cab_conf.to_main();
        match write_cabinet(&cab) {
            Ok(_) => println!("Applied Main preset to cabinet: {}", cab_path),
            Err(e) => eprintln!("Failed to save cabinet config for {}: {}", cab_path, e),
        }
    }
}

fn handle_preset_mirror(cmd: PresetMirrorArgs) {
    for cab_path in cmd.cabinet_paths {
        let mut cab = match build_cabinet_from_path(Path::new(&cab_path)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to build cabinet from path {}: {}", cab_path, e);
                continue;
            }
        };
        cab.cab_info.cab_conf.to_mirror();
        match write_cabinet(&cab) {
            Ok(_) => println!("Applied Main preset to cabinet: {}", cab_path),
            Err(e) => eprintln!("Failed to save cabinet config for {}: {}", cab_path, e),
        }
    }
}
