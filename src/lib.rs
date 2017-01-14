use std::process::Command;
use std::env;
use std::path::Path;

pub fn build(name: &str, arch: &str, file: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let o_name = format!("{}.o", name);
    let a_name = format!("lib{}.a", name);

    Command::new("nasm").args(&[file, &format!("-f{}", arch), "-o"])
        .arg(&format!("{}/{}", out_dir, o_name))
        .status().unwrap();

    Command::new("ar").args(&["crus", &a_name, &o_name])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}", name);
    println!("cargo:rerun-if-changed={}", file);
}
