use crate::logi::{gp::Group, mem::Member};

use std::process::Command;

impl Group {
    pub fn exec(&self, script: &String) {
        self.mems.iter().for_each(|mem| mem.exec(script));
    }
}

impl Member {
    fn exec(&self, script: &String) {
        let cd_dir = self.mem_info.cab_info.abs_path.as_path();

        // description: cd cd_dir & sh -c <Script>

        #[cfg(target_family = "unix")]
        {
            let status = Command::new("sh")
                .arg("-c")
                .arg(script)
                .current_dir(cd_dir)
                .status()
                .expect("failed to execute script");

            if !status.success() {
                eprintln!("Script failed with exit code {:?}", status.code());
            }
        }
        #[cfg(target_family = "windows")]
        {
            let status = Command::new("cmd")
                .arg("/C")
                .arg(script)
                .current_dir(cd_dir)
                .status()
                .expect("failed to execute script");

            if !status.success() {
                eprintln!("Script failed with exit code {:?}", status.code());
            }
        }
    }
}
