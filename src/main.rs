// src/main.rs
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
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

    loop {
        unsafe { core::arch::asm!("int 3") }
    }
}
