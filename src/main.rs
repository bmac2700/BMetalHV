// src/main.rs
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod arch;
mod bsp;
mod drivers;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" fn kernel_entry() -> ! {
    if arch::initialize().is_err() {}
    if bsp::initialize().is_err() {}

    loop {}
}
