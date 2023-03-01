fn main() {
    #[cfg(all(feature = "bsp_generic", feature = "arch_x86_64"))]
    println!("cargo:rustc-link-arg=-Tsrc/bsp/generic_x86_64/linker.ld")
}
