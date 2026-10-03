// build.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

fn main() {
    println!("cargo:rustc-link-arg=-Tsrc/bsp/link.ld");
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-search=src/bsp");
    println!("cargo:rerun-if-changed=src/bsp/link.ld");
    println!("cargo:rerun-if-changed=src/bsp/memory.ld");
    println!("cargo:rerun-if-changed=src/bsp/sections.ld");
}