pub extern "x86-interrupt" fn double_fault_handler(stack_frame: *const u8, _error_code: u64) -> ! {
    static MESSAGE: &[u8] = b"EXCEPTION: DOUBLE FAULT";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x4;
        }
    }
    loop {}
}