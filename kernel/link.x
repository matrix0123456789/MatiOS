ENTRY(_start)

MEMORY {
  ram : org = 0x9000, len = 0x1000000
}

SECTIONS {

  .text   : { *(.start) *(.text .text.*) }   > ram
  .rodata : { *(.rodata .rodata.*) } > dos
  .data   : { *(.data) }   > ram
  .bss    : { *(.bss) }    > dos
  .stack  : { *(.stack) }  > dos
  _heap = ALIGN(8);
}
