// this one's gonna get big...

pub const VER_REG: *mut u32 = 0x01C00024 as *mut u32;

pub const PD_CFG_REG: [*mut u32; 4] = [
  0x1c2086C as *mut u32, 0x1c20870 as *mut u32,
  0x1c20874 as *mut u32, 0x1c20878 as *mut u32
];
pub const PD_DATA_REG: *mut u32 = 0x1c2087c as *mut u32;

pub const PL_CFG_REG: [*mut u32; 4] = [
  0x01F02C00 as *mut u32, 0x01F02C04 as *mut u32,
  0x01F02C08 as *mut u32, 0x01F02C0C as *mut u32
];

