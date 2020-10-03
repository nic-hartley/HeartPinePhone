use super::*;

pub fn set(vibing: bool) {
  unsafe {
    *PD_CFG_REG[0] &= 0xfffff0ff;
    *PD_CFG_REG[0] |= 0x00000100;
  }

  unsafe {
    *PD_DATA_REG &= 0b11111111_11111111_11111111_11111011;
    *PD_DATA_REG |= (vibing as u32) << 2;
  }
}
