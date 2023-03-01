fn main() {
    println!("cargo:rustc-link-arg=-Tsrc/bsp/generic_x86_64/linker.ld")
}
