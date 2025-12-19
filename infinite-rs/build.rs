//! Originally from: <https://github.com/rfuzzo/red4lib>

use cmake::Config;
use std::path::Path;

fn main() {
    let kraken_path = Path::new("ext").join("kraken");
    let mut cfg = Config::new(kraken_path);

    let dst = cfg.build_target("kraken_static").build();

    if cfg!(target_os = "linux") {
        println!(
            "cargo:rustc-link-search=native={}/build/bin/CMake",
            dst.display()
        );
        println!("cargo:rustc-link-lib=static=kraken_static");
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if cfg!(windows) {
        let cmake_profile = cfg.get_profile();
        let link_path = format!("{}/build/bin/CMake/{}", dst.display(), cmake_profile);
        println!("cargo:rustc-link-search=native={}", link_path);
        println!("cargo:rustc-link-lib=static=kraken_static");
    }
}
