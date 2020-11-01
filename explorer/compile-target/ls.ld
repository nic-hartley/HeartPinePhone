ENTRY(_start)

MEMORY {
  sram_a1 (rx!w) : ORIGIN = 0x00010000, LENGTH = 0x00008000
  sram_c  (rw!x) : ORIGIN = 0x00018000, LENGTH = 0x00010000
  /* the actual SRAM-C region extends out through this MEMORY segment, but I want to reserve stack space explicitly */
  stack   (rw!x) : ORIGIN = 0x00028000, LENGTH = 0x0000B000
  mmio    (rw!x) : ORIGIN = 0x01C00000, LENGTH = 0x00400000
}

PHDRS {
  load_me PT_LOAD ;
}

SECTIONS {
  ALIGNMENT = 0x10;

  /* .header is just to ensure addresses are calculated with the eGON header in mind */
  /* note this _cannot_ just be a += in .text; a separate section carves out the "added" bytes */
  .header : {
    /* value is constant, based on the contents of mksunxiboot.c */
    . += 0x60;
  } >sram_a1 :load_me =0x00

  /* .text will be copied out into the binary and actually executed */
  .text : {
    hpprt.o(.text);
    *(.text .text.*);
    . = ALIGN(ALIGNMENT);
    *(.rodata .rodata.*);
  } >sram_a1 :load_me =0x00

  /* This bit isn't copied into the final firmware, it just generates the __bss symbols for hpprt.o. */
  .to_zero : {
    . = ALIGN(ALIGNMENT);
    __bss_start = .;
    *(.bss .bss.*);
    __bss_end = .;
  } >sram_c :load_me =0x00

  /* stack grows _down_, so start it at the end of the memory region */
  __stack_start = ORIGIN(stack) + LENGTH(stack);
  __stack_end = ORIGIN(stack);

  /* LD yells at me if I don't have .shstrtab, so */
  .shstrtab : {
    *(.shstrtab)
  } :NONE

  /* This is just so Ghidra will work right */
  .mmio : {
    . += LENGTH(mmio);
  } >mmio :load_me =0x00

  /DISCARD/ : { *(*) }
}

