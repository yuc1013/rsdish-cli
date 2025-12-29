use std::{fs, path::PathBuf};

use ignore::{DirEntry, WalkBuilder};
use tracing::{error, info};

use crate::logi::{gp::Group, mem::Member, vmem::VirtualMember};

impl Group {
    pub fn clean_from_vmem(&self, vmem: &VirtualMember) {
        let dsts = self
            .mems
            .iter()
            .filter(|mem| mem.mem_info.mem_conf.dst_option.enable);
        dsts.for_each(|dst| {
            info!("run clean for dst {:?}", dst.mem_info.cab_info.abs_path);
            dst.clean_from_vmem(vmem);
        });
    }
}

impl Member {
    fn clean_from_vmem(&self, vmem: &VirtualMember) {
        let dst_root = self.mem_info.cab_info.abs_path.as_path();

        let walker = WalkBuilder::new(dst_root)
            .add_custom_ignore_filename(env!("SRC_IGNORE_NAME"))
            .filter_entry(|ent: &DirEntry| {
                // default: ignore cabinet config
                !ent.file_name()
                    .to_string_lossy()
                    .eq(env!("CABINET_CONFIG_NAME"))
            })
            .build();
        let mut clean_queue: Vec<PathBuf> = Vec::new();

        for ent in walker {
            let ent = match ent {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to read entry: {}", e);
                    continue;
                }
            };

            let ent_meta = match fs::symlink_metadata(ent.path()) {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to read meta {}: {}", ent.path().display(), e);
                    continue;
                }
            };

            // symlink pass
            if ent_meta.is_symlink() {
                continue;
            }

            // folder pass
            if ent_meta.is_dir() {
                continue;
            }

            // ent must be "file"
            let file_abs_path = ent.path().to_path_buf();
            let file_rel_path = match file_abs_path.strip_prefix(dst_root) {
                Ok(t) => t,
                Err(e) => {
                    error!(
                        "Faild to calc relative path {} under {}: {}",
                        file_abs_path.display(),
                        dst_root.display(),
                        e
                    );
                    continue;
                }
            };

            let Some(exist_leaf) = vmem.virtual_tree.get(file_rel_path) else {
                return;
            };

            // clean if current is not highest in the tree
            if exist_leaf.priority < vmem.highest_priority {
                clean_queue.push(file_abs_path.clone());
            }
        }

        for path in clean_queue {
            match fs::remove_file(&path) {
                Ok(_) => {
                    info!("Removed low-priority file: {}", path.display());
                }
                Err(e) => {
                    error!("Failed to remove {}: {}", path.display(), e);
                }
            }
        }
    }
}
