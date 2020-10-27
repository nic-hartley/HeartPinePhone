ENTRY(_start)

MEMORY {
  sram_a1 : ORIGIN = 0x00010000, LENGTH = 0x8000
  sram_c  : ORIGIN = 0x00018000, LENGTH = 0x10000
  /* the actual SRAM-C region extends out through this MEMORY segment, but I want to reserve stack space explicitly */
  stack   : ORIGIN = 0x00028000, LENGTH = 0x0B000
}

SECTIONS {
  ALIGNMENT = 0x10;

  /* .text will be copied out into the binary and actually executed */
  .text : {
    hpprt.o(.text);
    *(.text .text.*);
    . = ALIGN(ALIGNMENT);
    *(.rodata .rodata.*);
  } >sram_a1 = 0x00

  /* This bit isn't copied into the final firmware, it just generates the __bss symbols for hpprt.o. */
  .to_zero : {
    . = ALIGN(ALIGNMENT);
    __bss_start = .;
    *(.bss .bss.*);
    __bss_end = .;
  } >sram_c = 0x00

  /* stack grows _down_, so start it at the end of the memory region */
  __stack_start = ORIGIN(stack) + LENGTH(stack);
  __stack_end = ORIGIN(stack);

  /* LD yells at me if I don't have .shstrtab, so */
  .shstrtab : {
    *(.shstrtab)
  }

  /DISCARD/ : { *(*) }
}

