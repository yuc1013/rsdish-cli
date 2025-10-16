use std::{fs, io, path::Path};

use tracing::{error, info};

use crate::{
    logi::{
        gp::Group,
        mem::Member,
        vmem::{VirtualLeaf, VirtualMember},
    },
    phy::cab_conf::SaveLevel,
};

impl Group {
    pub fn link_from_vmem(&self, vmem: &VirtualMember) {
        let link_dsts = self
            .mems
            .iter()
            .filter(|mem| mem.mem_info.mem_conf.link_option.enable);
        link_dsts.for_each(|link_dst| {
            info!("linking dst {:?}", link_dst.mem_info.cab_info.abs_path);
            link_dst.link_from_vmem(vmem);
        });
    }
}

impl Member {
    fn link_from_vmem(&self, vmem: &VirtualMember) {
        let link_dst_root = self.mem_info.cab_info.abs_path.as_path();
        let priority = self.mem_info.mem_conf.priority;

        for leaf in vmem.virtual_tree.values() {
            let target_abs_path = link_dst_root.join(leaf.file_rel_path.as_path());

            match fs::metadata(&target_abs_path) {
                Ok(_) => (),
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::NotFound {
                        error!("Failed to read meta {}: {}", target_abs_path.display(), e);
                        continue;
                    }
                    match SaveLevel::from(self.mem_info.mem_conf.dst_option.save_level) {
                        SaveLevel::DontSave => self.link_from_leaf(leaf, false),
                        SaveLevel::SaveHigher => {
                            self.link_from_leaf(leaf, priority < leaf.priority)
                        }
                        SaveLevel::SaveHigherEqual => {
                            self.link_from_leaf(leaf, priority <= leaf.priority)
                        }
                        SaveLevel::SaveAll => self.link_from_leaf(leaf, true),
                        _ => (),
                    }
                }
            };
        }
    }

    fn link_from_leaf(&self, leaf: &VirtualLeaf, condition: bool) {
        if !condition {
            return;
        }

        let link_dst_root = &self.mem_info.cab_info.abs_path;
        let target_abs_path = link_dst_root.join(&leaf.file_rel_path);

        match create_symlink(leaf.file_abs_path.as_path(), target_abs_path.as_path()) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Failed to link from original {} to link {}: {}",
                    leaf.file_abs_path.display(),
                    target_abs_path.display(),
                    e
                );
                return;
            }
        }
    }
}

pub fn create_symlink(original: &Path, link: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(original, link)
    }

    #[cfg(windows)]
    {
        if original.is_dir() {
            std::os::windows::fs::symlink_dir(original, link)
        } else {
            std::os::windows::fs::symlink_file(original, link)
        }
    }
}
