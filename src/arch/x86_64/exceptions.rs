//https://wiki.osdev.org/Interrupt_Descriptor_Table

#[derive(Clone, Copy)]
pub struct InterruptTableEntryOptions(u16);

impl InterruptTableEntryOptions {

    #[inline]
    const fn minimal() -> Self {
        InterruptTableEntryOptions(0b00000001_11000000)
    }

}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptTableEntry {
    isr_low: u16,
    kernel_cs: u16, //GDT Segment
    options: InterruptTableEntryOptions,
    isr_mid: u16,
    isr_high: u32,
    reserved: u32,
}

impl InterruptTableEntry {
    #[inline]
    const fn default() -> Self {
        Self {
            kernel_cs: 0,
            isr_low: 0,
            isr_mid: 0,
            isr_high: 0,
            options: InterruptTableEntryOptions::minimal(),
            reserved: 0
        }
    }
}

#[repr(C)]
pub struct InterruptDescriptorTable {
    interrupts: [InterruptTableEntry; 256],
}

impl InterruptDescriptorTable {
    #[inline]
    const fn new() -> Self {
        Self {
            interrupts: [InterruptTableEntry::default(); 256],
        }
    }

    #[inline]
    pub fn load(&'static self) {
        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) self, options(readonly, nostack, preserves_flags));  
        }
    }
}

static mut INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn initialize_exceptions() -> Result<(), ()> {
    unsafe {INTERRUPT_DESCRIPTOR_TABLE.load()}
    Ok(())
}