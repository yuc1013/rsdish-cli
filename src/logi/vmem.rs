use ignore::{DirEntry, WalkBuilder};
use tracing::{error, info};

use std::{collections::BTreeMap, env, fs, path::PathBuf};

use crate::logi::{gp::Group, mem::Member};

pub struct VirtualMember {
    pub virtual_tree: BTreeMap<PathBuf, VirtualLeaf>,
}

#[derive(Debug, Clone)]
pub struct VirtualLeaf {
    pub file_rel_path: PathBuf,
    pub file_abs_path: PathBuf,
    pub priority: i32,
}

pub fn build_virtual_member_from_group(gp: &Group) -> VirtualMember {
    let mut vmem = VirtualMember {
        virtual_tree: BTreeMap::new(),
    };

    let srcs = gp
        .mems
        .iter()
        .filter(|mem| mem.mem_info.mem_conf.src_option.enable);

    srcs.for_each(|mem: &Member| {
        info!(
            "learn src for virtual member {:?}",
            mem.mem_info.cab_info.abs_path
        );
        vmem.learn(mem);
    });

    vmem
}

impl VirtualMember {
    pub fn learn(&mut self, src: &Member) {
        let src_root = src.mem_info.cab_info.abs_path.as_path();

        let walker = WalkBuilder::new(src_root)
            .add_custom_ignore_filename(env!("SRC_IGNORE_NAME"))
            .filter_entry(|ent: &DirEntry| {
                // default: ignore cabinet config
                !ent.file_name().to_string_lossy().eq(env!("CABINET_CONFIG_NAME"))
            })
            .build();

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
            let file_rel_path = match file_abs_path.strip_prefix(src_root) {
                Ok(t) => t,
                Err(e) => {
                    error!(
                        "Faild to calc relative path {} under {}: {}",
                        file_abs_path.display(),
                        src_root.display(),
                        e
                    );
                    continue;
                }
            };
            let priority = src.mem_info.mem_conf.priority;
            self.learn_from_leaf(&VirtualLeaf {
                file_rel_path: file_rel_path.to_path_buf(),
                file_abs_path: file_abs_path,
                priority: priority,
            });
        }
    }

    fn learn_from_leaf(&mut self, leaf: &VirtualLeaf) {
        let Some(exist_leaf) = self.virtual_tree.get(&leaf.file_rel_path.to_path_buf()) else {
            // learn if not exist
            self.virtual_tree
                .insert(leaf.file_rel_path.clone(), leaf.clone());
            return;
        };

        // learn if leaf priority is higher
        if exist_leaf.priority < leaf.priority {
            self.virtual_tree
                .insert(leaf.file_rel_path.clone(), leaf.clone());
            return;
        }
    }
}
