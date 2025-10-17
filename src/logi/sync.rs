use tracing::{error, info};

use std::{
    env, fs,
    process::{Command, Stdio},
};

use crate::{
    logi::{
        gp::Group,
        mem::Member,
        vmem::{VirtualLeaf, VirtualMember},
    },
    phy::cab_conf::{CoverLevel, SaveLevel},
};

impl Group {
    pub fn sync_from_vmem(&self, vmem: &VirtualMember) {
        let dsts = self
            .mems
            .iter()
            .filter(|mem| mem.mem_info.mem_conf.dst_option.enable);
        dsts.for_each(|dst| {
            info!("run sync for dst {:?}", dst.mem_info.cab_info.abs_path);
            dst.sync_from_vmem(vmem);
        });
    }
}

impl Member {
    fn sync_from_vmem(&self, vmem: &VirtualMember) {
        let dst_root = self.mem_info.cab_info.abs_path.as_path();
        let priority = self.mem_info.mem_conf.priority;

        for leaf in vmem.virtual_tree.values() {
            let target_abs_path = dst_root.join(leaf.file_rel_path.as_path());

            match fs::metadata(&target_abs_path) {
                Ok(_) => match CoverLevel::from(self.mem_info.mem_conf.dst_option.cover_level) {
                    CoverLevel::DontCover => self.sync_from_leaf(leaf, false),
                    CoverLevel::HigherCover => self.sync_from_leaf(leaf, priority < leaf.priority),
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
                error!("Failed to get parent of {}", target_abs_path.display());
                return;
            }
        };

        let runtime_params = env::var("RSDISH_RUNTIME_PARAMS").unwrap_or_default();
        let rclone = env::var("RCLONE_PATH").unwrap_or_default();
        let rclone = if rclone.is_empty() { "rclone" } else { &rclone };

        // collect parsed args
        let mut args = vec![
            "sync".to_string(),
            leaf.file_abs_path.to_string_lossy().to_string(),
            target_folder.to_string_lossy().to_string(),
        ];

        for param_str in [&runtime_params, &self.mem_info.mem_conf.dst_option.params] {
            if !param_str.is_empty() {
                if let Ok(mut v) = shell_words::split(param_str) {
                    args.append(&mut v);
                }
            }
        }
        info!("rclone args = {} {}", rclone, args.join(" "));

        let mut child = Command::new(&rclone)
            .args(&args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to spawn rclone");

        let status = child.wait().expect("failed to wait on rclone");
        if !status.success() {
            error!(
                "rclone failed for {} with status {}",
                leaf.file_rel_path.display(),
                status
            );
        }
    }
}
