extern crate cpp_build;

use std::convert::TryInto;
use std::{env, fs, path, process};

fn main() {
    let mut out_dir: path::PathBuf = env::var("OUT_DIR").unwrap().try_into().unwrap();
    out_dir.push("voro++");
    let _ = fs::create_dir(&out_dir);
    println!("cargo:rustc-link-lib=static=voro++");
    println!(
        "cargo:rustc-link-search={}",
        out_dir.as_path().to_str().unwrap()
    );
    // Copy the source files into the output directory.
    for entry in fs::read_dir("src").unwrap() {
        let src_file = entry.unwrap().path();
        if src_file.as_path().is_file() {
            let mut dst_file = out_dir.clone();
            dst_file.push(src_file.as_path().file_name().unwrap());
            fs::copy(src_file, &dst_file).unwrap();
        }
    }
    let mut parent = out_dir.clone();
    parent.push("../config.mk");
    fs::copy("config.mk", parent).unwrap();
    process::Command::new("make")
        .args(&[
            "-C",
            out_dir.as_path().to_str().unwrap(),
            "-j",
            "CFLAGS=-fPIC",
        ])
        .status()
        .unwrap();
    cpp_build::Config::new().include("src").build("src/lib.rs");
}
