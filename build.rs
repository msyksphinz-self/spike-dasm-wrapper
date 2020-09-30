extern crate cc;
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rustc-link-lib=c++");

    let mut mkdir = Command::new("mkdir")
        .arg("-p")
        .arg("riscv-isa-sim/build")
        .stdout(Stdio::piped())
        .spawn().expect("failed to run");
    mkdir.wait().unwrap();
    let mut configure = Command::new("../configure")
        .current_dir("riscv-isa-sim/build")
        .stdout(Stdio::piped())
        .spawn().expect("failed to run");
    configure.wait().unwrap();
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-std=c++17")
        .file("riscv-isa-sim/disasm/disasm.cc")
        .file("riscv-isa-sim/disasm/regnames.cc")
        .file("src/helper.cpp")
        .include("riscv-isa-sim/riscv")
        .include("riscv-isa-sim/softfloat")
        .include("riscv-isa-sim/build")
        .compile("spike-dasm");
}
