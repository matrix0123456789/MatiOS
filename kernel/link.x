ENTRY(_start)

MEMORY {
  ram : org = 0x9000, len = 0x1000000
}

SECTIONS {

  .text   : { *(.main) *(.text .text.*) }   > ram
  .rodata : { *(.rodata .rodata.*) } > ram
  .data   : { *(.data .data.*) }   > ram
  .bss    : { *(.bss) }    > ram
  .stack  : { *(.stack) }  > ram
  _heap = ALIGN(8);
}
