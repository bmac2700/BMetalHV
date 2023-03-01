#[cfg(all(feature = "bsp_generic", feature = "arch_x86_64"))]
mod generic_x86_64;

#[cfg(all(feature = "bsp_generic", feature = "arch_x86_64"))]
pub use generic_x86_64::*;
