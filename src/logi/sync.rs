use tracing::{error, info};

use std::{
    fs::{self, Metadata},
    time::SystemTime,
};

use crate::{
    logi::{
        gp::Group,
        mem::Member,
        vmem::{VirtualLeaf, VirtualMember},
    },
    phy::cab_conf::{CoverLevel, SaveLevel},
};

pub fn need_sync(src: &VirtualLeaf, dst_meta: &Metadata, strict: bool) -> bool {
    if strict {
        return true;
    }

    let src_meta = match fs::metadata(&src.file_abs_path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let src_size = src_meta.len();
    let dst_size = dst_meta.len();

    let src_mtime = src_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let dst_mtime = dst_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    if src_size != dst_size || src_mtime > dst_mtime {
        true
    } else {
        false
    }
}

impl Group {
    pub fn sync_from_vmem(&self, vmem: &VirtualMember, strict: bool) {
        let dsts = self
            .mems
            .iter()
            .filter(|mem| mem.mem_info.mem_conf.dst_option.enable);
        dsts.for_each(|dst| {
            info!("run sync for dst {:?}", dst.mem_info.cab_info.abs_path);
            dst.sync_from_vmem(vmem, strict);
        });
    }
}

impl Member {
    fn sync_from_vmem(&self, vmem: &VirtualMember, strict: bool) {
        let dst_root = self.mem_info.cab_info.abs_path.as_path();
        let priority = self.mem_info.mem_conf.priority;

        for leaf in vmem.virtual_tree.values() {
            let target_abs_path = dst_root.join(leaf.file_rel_path.as_path());

            match fs::metadata(&target_abs_path) {
                Ok(t) => match CoverLevel::from(self.mem_info.mem_conf.dst_option.cover_level) {
                    CoverLevel::DontCover => self.sync_from_leaf(leaf, false),
                    CoverLevel::HigherCover => self.sync_from_leaf(
                        leaf,
                        priority < leaf.priority && need_sync(leaf, &t, strict),
                    ),
                    _ => (),
                },
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::NotFound {
                        error!("Failed to read meta {}: {}", target_abs_path.display(), e);
                        continue;
                    }
                    match SaveLevel::from(self.mem_info.mem_conf.dst_option.save_level) {
                        SaveLevel::DontSave => self.sync_from_leaf(leaf, false),
                        SaveLevel::SaveHigher => {
                            self.sync_from_leaf(leaf, priority < leaf.priority)
                        }
                        SaveLevel::SaveHigherEqual => {
                            self.sync_from_leaf(leaf, priority <= leaf.priority)
                        }
                        SaveLevel::SaveAll => self.sync_from_leaf(leaf, true),
                        _ => (),
                    }
                }
            };
        }
    }

    fn sync_from_leaf(&self, leaf: &VirtualLeaf, condition: bool) {
        if !condition {
            return;
        }

        let dst_root = &self.mem_info.cab_info.abs_path;
        let target_abs_path = dst_root.join(&leaf.file_rel_path);

        let target_folder = match target_abs_path.parent() {
            Some(p) => p,
            None => {
                error!("Failed to get parent of {:?}", target_abs_path);
                return;
            }
        };

        match fs::create_dir_all(target_folder) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to create target folder {:?}: {}", target_folder, e);
                return;
            }
        }

        match fs::copy(&leaf.file_abs_path, &target_abs_path) {
            Ok(_) => {
                info!(
                    "Synced file {:?} to {:?}",
                    leaf.file_abs_path, target_abs_path
                );
            }
            Err(e) => {
                error!(
                    "Failed to copy file {:?} to {:?}: {}",
                    leaf.file_abs_path, target_abs_path, e
                );
            }
        }
    }
}
