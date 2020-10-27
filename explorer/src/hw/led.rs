use super::*;

pub enum Color {
  Red =     0b010 << 18,
  Yellow =  0b011 << 18,
  Green =   0b001 << 18,
  Cyan =    0b101 << 18,
  Blue =    0b100 << 18,
  Magenta = 0b110 << 18,
  Black =   0b000 << 18,
  White =   0b111 << 18,
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

pub fn init() {
  PD_CFG_REG.index(2).update(|r| r & 0xfff000ff | 0x00011100);
}

pub fn set(color: Color) {
  PD_DATA_REG.update(|r| (r & !0x001c0000) | (color as u32));
}
