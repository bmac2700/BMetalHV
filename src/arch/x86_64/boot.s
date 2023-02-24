.section .text
.code32
//------------------------------------------------------------------------------
// pub fn _start()
//------------------------------------------------------------------------------
.global	start
start:
	mov dword ptr [0xb8000], 0x2f4b2f4f
    hlt

.size	start, . - start
.type	start, function