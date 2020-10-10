use super::*;

pub enum Color {
  Red =     0b00000000_00001000_00000000_00000000,
  Yellow =  0b00000000_00001100_00000000_00000000,
  Green =   0b00000000_00000100_00000000_00000000,
  Cyan =    0b00000000_00010100_00000000_00000000,
  Blue =    0b00000000_00010000_00000000_00000000,
  Magenta = 0b00000000_00011000_00000000_00000000,
  Black =   0b00000000_00000000_00000000_00000000,
  White =   0b00000000_00011100_00000000_00000000,
}

impl Color {
  pub const fn of_bits(r: bool, g: bool, b: bool) -> Color {
    match (r, g, b) {
      (true, false, false) => Color::Red,
      (true, true, false) => Color::Yellow,
      (false, true, false) => Color::Green,
      (false, true, true) => Color::Cyan,
      (false, false, true) => Color::Blue,
      (true, false, true) => Color::Magenta,
      (false, false, false) => Color::Black,
      (true, true, true) => Color::White,
    }
  }
}

pub fn set(color: Color) {
  unsafe {
    *PD_CFG_REG[2] = *PD_CFG_REG[2] & 0xfff000ff | 0x00011100;
  }

  unsafe {
    *PD_DATA_REG = *PD_DATA_REG & 0b11111111_11100011_11111111_11111111 | color as u32;
  }
}
