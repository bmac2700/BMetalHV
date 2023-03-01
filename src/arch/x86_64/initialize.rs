pub fn initialize() -> Result<(), ()> {
    super::exceptions::initialize_exceptions()?;
    super::paging::initialize_paging()?;

    static HELLO: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    Ok(())
}
