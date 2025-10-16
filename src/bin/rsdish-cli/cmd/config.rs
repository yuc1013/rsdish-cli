use clap::{Args};
use rsdish::user::user_conf::user_conf_path;

#[derive(Debug, Args)]
#[command(about = "Print config path.")]
pub struct ConfigCmd {}

pub fn handle_config(_: ConfigCmd) {
  let user_conf_path = user_conf_path().unwrap();
  println!("{:?}", user_conf_path);
}