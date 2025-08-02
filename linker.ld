SECTIONS {
    . = 0x100000;
    .text : {
        *(.text.entry)
        *(.text*)
    }
    . = ALIGN(4K);
    .rodata : {
        *(.rodata*)
    }
    . = ALIGN(4K);
    .data : {
        *(.data*)
    }
    . = ALIGN(4K);
    .bss : {
        *(.bss*)
    }
}
