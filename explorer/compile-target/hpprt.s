.extern k_main, __bss_start, __bss_end, __stack_start
.global _start

.macro delay amt=50000 reg=r2
    MOV \reg, #\amt
delay_\@:
    SUBS \reg, #1
    BNE delay_\@
.endm

.text
@ _start HAS TO be at the beginning of .text in this file.
@ (or, well, technically, the boot code needs to start at the beginning of .text)
@ (but I'm using _start to mark that)
_start:
    @ turn on red LED
    @ configure pins as GPIO
    MOVW r0, #0x0874
    MOVT r0, #0x01C2
    MOVW r1, #0x1777
    MOVT r1, #0x7777
    STR r1, [r0]
    @ turn on red pin
    MOVW r1, #0x0000
    MOVT r1, #0x0008
    STR r1, [r0, #0x8]

    @ zero out __bss_start to __bss_end
    @ (it might overrun the end, but who cares? We _know_ there's nothing there yet.)
    LDR r0, bss_start
    LDR r1, bss_end
    MOV r2, #0
    MOV r3, #0
next_bss:
    @ 32-bit wide registers and 2 of them = 64 bits = 8 bytes
    STRD r2, r3, [r0], #8
    CMP r0, r1
    @ if r0 < r1, we haven't gotten there, do some more
    BLT next_bss
    @ bss now cleared

    @ set stack pointer
    LDR sp, stack_start

    @ branch into the entry point
    @ (called k_main so it's not confused with an application's `main`)
    B k_main

bss_start:
    .word __bss_start
bss_end:
    .word __bss_end
stack_start:
    .word __stack_start - 4
