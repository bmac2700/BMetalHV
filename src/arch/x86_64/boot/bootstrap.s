.section .text
.code32
//------------------------------------------------------------------------------
// pub fn _start()
//------------------------------------------------------------------------------
.global	start
start:
    mov esp, offset stack_top // Setup stack

    call setup_pml4
    call setup_pdpt
    call setup_pdt
    call enable_paging

    lgdt [gdt64.pointer]

    ljmp 8, offset long_mode_start
    hlt

.size	start, . - start
.type	start, function

.set PRESENT_FLAG, 1 << 0
.set READWRITE_FLAG, 1 << 1
.set PAGE_FLAG, 1 << 7

setup_pml4:
    push eax

    mov eax, offset pdpt_table

    or eax, PRESENT_FLAG
    or eax, READWRITE_FLAG

    mov pml4t_table, eax

    pop eax
    
    ret

setup_pdpt:
    push eax

    mov eax, offset pdt_table

    or eax, PRESENT_FLAG
    or eax, READWRITE_FLAG

    mov pdpt_table, eax

    pop eax
    
    ret

setup_pdt:
    push eax
    push ecx

    xor ecx, ecx // Reset ECX to 0

.map_pdt_table:
    mov eax, 0x200000
    mul ecx

    or eax, PRESENT_FLAG
    or eax, READWRITE_FLAG
    or eax, PAGE_FLAG

    mov [pdt_table + ecx*8], eax

    inc ecx
    cmp ecx, 512
    jne .map_pdt_table

    pop ecx
    pop eax
    
    ret

enable_paging:
    push eax
    push ecx

    mov eax, offset pml4t_table
    mov cr3, eax

    xor eax, eax

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    pop ecx
    pop eax
    ret

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

.section .rodata
gdt64:
    .8byte 0 // zero entry
gdt64.code:
    .8byte (1<<43) | (1<<44) | (1<<47) | (1<<53) // code segment
gdt64.pointer:
    .2byte . - gdt64 - 1
    .8byte gdt64


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
    cli
    hlt