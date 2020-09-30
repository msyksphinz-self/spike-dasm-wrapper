extern crate cc;

fn main() {
    println!("cargo:rustc-link-lib=c++");

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
        .compile("spike-dasm")
}
