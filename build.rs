extern crate cpp_build;

use std::convert::TryInto;
use std::{env, fs, path, process};

fn main() {
    //
    let src_dir = std::env::current_dir().unwrap();
    //
    let mut out_dir: path::PathBuf = env::var("OUT_DIR").unwrap().try_into().unwrap();
    out_dir.push("voro++");
    let _ = fs::create_dir(&out_dir);
    //
    process::Command::new("cmake")
        .current_dir(&out_dir)
        .args(&[src_dir.as_path().to_str().unwrap()])
        .status()
        .unwrap();
    process::Command::new("make")
        .current_dir(&out_dir)
        .args(&["-j"])
        .status()
        .unwrap();
    //
    println!("cargo:rustc-link-lib=static=voro++");
    println!(
        "cargo:rustc-link-search={}",
        out_dir.as_path().to_str().unwrap()
    );
    //
    cpp_build::Config::new().include("src").build("src/lib.rs");
}
