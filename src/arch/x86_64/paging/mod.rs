pub fn initialize_paging() -> Result<(), ()> {
    let mut cr3: u64 = 0;

    unsafe { core::arch::asm!("mov {cr3}, cr3", cr3 = inout(reg) cr3) };

    
    Ok(())
}
