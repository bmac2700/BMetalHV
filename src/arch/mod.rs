#[cfg(all(feature = "arch_x86_64"))]
mod x86_64;

#[cfg(all(feature = "arch_x86_64"))]
pub use x86_64::*;
