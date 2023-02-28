use core::arch::global_asm;

global_asm!(include_str!("multiboot.s"));

global_asm!(include_str!("bootstrap.s"));

#[no_mangle]
unsafe extern "C" fn _start_rust() -> ! {
    crate::kernel_entry()
}
