use super::*;

pub fn set(vibing: bool) {
  PD_CFG_REG.index(0).update(|r| r & 0xfffff0ff | 0x00000100);

  let new_bit = if vibing { 1 } else { 0 } << 2;
  PD_DATA_REG.update(|r| r & 0b11111111_11111111_11111111_11111011 | new_bit);
}
