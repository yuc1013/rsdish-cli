use thiserror::Error;
use tracing::error;

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::phy::cab::{Cabinet, build_cabinet_from_path};

#[derive(Debug, Error)]
pub enum StorageError {}

#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub abs_path: PathBuf,
}

#[derive(Debug)]
pub struct Storage {
    pub stg_info: StorageInfo,
    // relations
    pub cabs: Vec<Cabinet>,
}

pub fn build_storages_from_paths(stg_abs_paths: &Vec<&Path>) -> Vec<Storage> {
    let mut stgs: Vec<Storage> = Vec::new();
    // ex: /System/Volumes/Samsung SSD, D:\
    for stg_abs_path in stg_abs_paths {
        let stg_ents = match fs::read_dir(stg_abs_path) {
            Ok(t) => t,
            Err(e) => {
                error!(
                    "Failed to read storage path {}: {}",
                    stg_abs_path.display(),
                    e
                );
                continue;
            }
        };

        let mut cabs: Vec<Cabinet> = Vec::new();
        // ex: /System/Volumes/Samsung SSD/anime123/, D:\game\
        for stg_ent in stg_ents {
            let f = match stg_ent {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to read entry in {}: {}", stg_abs_path.display(), e);
                    continue;
                }
            };
            let f_path = f.path();

            let f_type = match f.file_type() {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to read entry in {}: {}", f_path.display(), e);
                    continue;
                }
            };

            if f_type.is_dir() {
                let cab = match build_cabinet_from_path(f_path.as_path()) {
                    Ok(t) => t,
                    Err(e) => {
                        error!("Failed to build cabinet from {}: {}", f_path.display(), e);
                        continue;
                    }
                };

                cabs.push(cab);
            }
        }

        if cabs.len() > 0 {
            let stg: Storage = Storage {
                stg_info: StorageInfo {
                    abs_path: stg_abs_path.to_path_buf(),
                },
                cabs: cabs,
            };
            stgs.push(stg);
        }
    };
    stgs
}
