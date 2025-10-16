use crate::phy::{cab::CabinetInfo, cab_conf::MemberConfig, stg::StorageInfo};

#[derive(Debug)]
pub struct Member {
  pub mem_info: MemberInfo,
  // relations
}

#[derive(Debug)]
pub struct MemberInfo {
  pub mem_conf: MemberConfig,
  pub cab_info: CabinetInfo,
  pub stg_info: StorageInfo,
}