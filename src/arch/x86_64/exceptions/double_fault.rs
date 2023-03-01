pub extern "x86-interrupt" fn double_fault_handler(stack_frame: *const u8, _error_code: u64) -> ! {
    unsafe { core::arch::asm!("xor rax, rax", "lidt [rax]", "int 3") }; //Restart the CPU by causing a triple fault
    loop {}
}
