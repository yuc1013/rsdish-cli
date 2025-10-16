use std::path::PathBuf;

use sysinfo::Disks;

pub fn disks()->Vec<PathBuf> {
    let dks = Disks::new_with_refreshed_list();

    dks.iter()
       .map(|dk|dk.mount_point().to_path_buf())
       .collect()
}