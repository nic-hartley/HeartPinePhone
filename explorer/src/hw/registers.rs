// this one's gonna get big...
// TODO: Organize
// - Structs/macros to hold/define common patterns
// - Base + offset? (write out the math from the manual instead of pre-doing it)
// NOTE: DO NOT want to make this too complex
// should be basically just holding lots of numbers
// no business logic in here
// TODO: More efficient ways?
// - *mut [u32; 4] instead of [*mut u32; 4]? syntax was ugly last time but doable?
// - `repr(C)` structs?
// TODO: Ensure all reads/writes are volatile.
// - Wrapper struct around pointers?

pub const VER_REG: *mut u32 = 0x01C00024 as *mut u32;

pub const PB_CFG_REG: [*mut u32; 2] = [
  0x01c20800 as *mut u32, 0x01c20804 as *mut u32,
];

pub const PB_DATA_REG: *mut u32 = 0x01c20810 as *mut u32;

pub const PD_CFG_REG: [*mut u32; 4] = [
  0x01c2086C as *mut u32, 0x01c20870 as *mut u32,
  0x01c20874 as *mut u32, 0x01c20878 as *mut u32
];
pub const PD_DATA_REG: *mut u32 = 0x01c2087c as *mut u32;

pub const UART0_DAT: *mut u32 = 0x01C28000 as *mut u32;
pub const UART0_FCR: *mut u32 = 0x01C28008 as *mut u32;
pub const UART0_LCR: *mut u32 = 0x01C2800C as *mut u32;
pub const UART0_SCH: *mut u32 = 0x01C2801C as *mut u32;
pub const UART0_USR: *mut u32 = 0x01C2807C as *mut u32;
pub const UART0_TFL: *mut u32 = 0x01C28080 as *mut u32;
pub const UART0_RFL: *mut u32 = 0x01C28084 as *mut u32;

pub const PL_CFG_REG: [*mut u32; 4] = [
  0x01F02C00 as *mut u32, 0x01F02C04 as *mut u32,
  0x01F02C08 as *mut u32, 0x01F02C0C as *mut u32
];

