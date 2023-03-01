//https://wiki.osdev.org/Interrupt_Descriptor_Table

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IDTEntry {
    base_low: u16,
    segment_selector: u16,
    ist: u8,
    attributes: u8,
    base_mid: u16,
    base_high: u32,
    reserved: u32,
}

impl IDTEntry {
    const fn default() -> Self {
        Self {
            base_low: 0,
            segment_selector: 0,
            ist: 0,
            attributes: 0b0000_1110,
            base_mid: 0,
            base_high: 0,
            reserved: 0,
        }
    }

    fn set_handler(&mut self, address: u64) {
        self.segment_selector = 0x8;
        self.base_low = address as u16;
        self.base_mid = (address >> 16) as u16;
        self.base_high = (address >> 32) as u32;

        self.attributes |= 0b1000_0000
    }
}

#[repr(C, packed)]
pub struct IDT {
    pub interrupts: [IDTEntry; 256],
}

impl IDT {
    const fn new() -> Self {
        Self {
            interrupts: [IDTEntry::default(); 256],
        }
    }
}

impl IDT {
    fn pointer(&self) -> IDTPointer {
        use core::mem::size_of;
        IDTPointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        }
    }
    pub fn load(&'static self) {
        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &self.pointer(), options(readonly, nostack, preserves_flags));
        }
    }
}

#[repr(C, packed)]
pub struct IDTPointer {
    limit: u16,
    base: u64,
}

static mut INTERRUPT_DESCRIPTOR_TABLE: IDT = IDT::new();

extern "x86-interrupt" fn double_fault_handler(stack_frame: *const u8, _error_code: u64) -> ! {
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

pub fn initialize_exceptions() -> Result<(), ()> {
    unsafe {
        //Double fault
        INTERRUPT_DESCRIPTOR_TABLE.interrupts[8].set_handler(double_fault_handler as u64);
    }
    unsafe { INTERRUPT_DESCRIPTOR_TABLE.load() };
    unsafe { core::arch::asm!("sti") };
    Ok(())
}
