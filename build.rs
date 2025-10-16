fn main() {
    let envs = [
        ("APP_NAME", "rsdish"), 
        ("APP_CONFIG_NAME", "rsdish.config"),
        ("CABINET_CONFIG_NAME", "rsdish.cabinet.toml"),
    ];
    
    for (k, v) in envs {
        println!("cargo:rustc-env={}={}", k, v);
    }

    println!("cargo:rerun-if-changed=build.rs");
}
