pub fn set(vibing: bool) {
  unsafe {
    *super::PD_CFG_REG[0] &= 0xfffff0ff;
    *super::PD_CFG_REG[0] |= 0x00000100;
  }

  unsafe {
    *super::PD_DATA_REG &= 0xffffffb;
    *super::PD_DATA_REG |= (vibing as u32) << 2;
  }
}
