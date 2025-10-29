use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CabinetConfigError {
    #[error("Group uuid is empty")]
    EmptyGroupUuid,
    #[error("Invalid cover level {0}")]
    InvalidCoverLevel(i32),
    #[error("Invalid save level {0}")]
    InvalidSaveLevel(i32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CabinetConfig {
    pub note: Option<String>,
    pub memberships: Vec<MemberConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberConfig {
    pub group_uuid: String,
    pub priority: i32,
    pub src_option: SrcOption,
    pub dst_option: DstOption,
    pub link_option: LinkOption,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SrcOption {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DstOption {
    pub enable: bool,
    pub cover_level: i32,
    pub save_level: i32,
    pub params: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkOption {
    pub enable: bool,
    pub save_level: i32,
}

pub enum CoverLevel {
    DontCover = 0,
    HigherCover = 1,
    Unknown,
}

impl From<i32> for CoverLevel {
    fn from(value: i32) -> Self {
        match value {
            0 => CoverLevel::DontCover,
            1 => CoverLevel::HigherCover,
            _ => CoverLevel::Unknown,
        }
    }
}

pub enum SaveLevel {
    DontSave = 0,
    SaveHigher = 1,
    SaveHigherEqual = 2,
    SaveAll = 3,
    Unknown,
}

impl From<i32> for SaveLevel {
    fn from(value: i32) -> Self {
        match value {
            0 => SaveLevel::DontSave,
            1 => SaveLevel::SaveHigher,
            2 => SaveLevel::SaveHigherEqual,
            3 => SaveLevel::SaveAll,
            _ => SaveLevel::Unknown,
        }
    }
}

pub fn default_cabinet_config() -> CabinetConfig {
    CabinetConfig {
        note: Some(String::from("New Cabinet")),
        memberships: vec![],
    }
}

pub fn default_membership() -> MemberConfig {
    MemberConfig {
        group_uuid: Uuid::now_v7().to_string(),
        priority: 0,
        src_option: SrcOption {
            enable: false,
        },
        dst_option: DstOption {
            enable: false,
            cover_level: CoverLevel::DontCover as i32,
            save_level: SaveLevel::DontSave as i32,
            params: String::new(),
        },
        link_option: LinkOption { enable: false, save_level: SaveLevel::DontSave as i32 },
    }
}

impl CabinetConfig {
    pub fn verify(&self) -> Result<(), CabinetConfigError> {
        self.memberships.iter().try_for_each(|m| {
            if m.group_uuid.trim().is_empty() {
                return Err(CabinetConfigError::EmptyGroupUuid);
            }

            if let CoverLevel::Unknown = CoverLevel::from(m.dst_option.cover_level) {
                return Err(CabinetConfigError::InvalidCoverLevel(
                    m.dst_option.cover_level,
                ));
            }

            if let SaveLevel::Unknown = SaveLevel::from(m.dst_option.save_level) {
                return Err(CabinetConfigError::InvalidSaveLevel(
                    m.dst_option.save_level,
                ));
            }
            Ok(())
        })
    }
}

impl CabinetConfig {
    pub fn to_gate(&mut self) {
        self.memberships.iter_mut().for_each(|m| {
            m.priority = 0;
            m.src_option.enable = false;
            m.dst_option.enable = false;
            m.link_option.enable = true;
            m.link_option.save_level = SaveLevel::SaveAll as i32;
        });
    }

    pub fn to_main(&mut self) {
        self.memberships.iter_mut().for_each(|m| {
            m.priority = 3;
            m.src_option.enable = true;
            m.dst_option.enable = true;
            m.link_option.enable = false;
            m.dst_option.cover_level = CoverLevel::HigherCover as i32;
            m.dst_option.save_level = SaveLevel::SaveHigherEqual as i32;
        });
    }

    pub fn to_mirror(&mut self) {
        self.memberships.iter_mut().for_each(|m| {
            m.priority = 1;
            m.src_option.enable = true;
            m.dst_option.enable = true;
            m.link_option.enable = false;
            m.dst_option.cover_level = CoverLevel::HigherCover as i32;
            m.dst_option.save_level = SaveLevel::SaveHigherEqual as i32;
        });
    }
}