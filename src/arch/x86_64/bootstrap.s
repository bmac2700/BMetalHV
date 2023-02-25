.section .text
.code32
//------------------------------------------------------------------------------
// pub fn _start()
//------------------------------------------------------------------------------
.global	start
start:
    mov esp, offset stack_top // Setup stack

    mov dword ptr [0xb8000], 0x2f4b2f4f
    hlt

.size	start, . - start
.type	start, function

.section .bss
.align 4096

pml4t_table:
    .skip 4096
pdpt_table:
    .skip 4096
pdt_table:
    .skip 4096
pt_table:
    .skip 4096

.section .bss
.align 16
stack_bottom:
    .skip 16384 # 16 KiB
stack_top:

.section .text
.code64
long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    call _start_rust
    hlt