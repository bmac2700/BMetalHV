.section .multiboot_header
.align 8
header_start:
    .4byte 0xe85250d6                // magic number (multiboot 2)
    .4byte 0                         // architecture 0 (protected mode i386)
    .4byte header_end - header_start // header length
    .4byte 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) // checksum

    // insert optional multiboot tags here

    // required end tag
    .2byte 0    // type
    .2byte 0    // flags
    .4byte 8    // size
header_end: