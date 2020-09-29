pub enum Color {
  Red = 0b010,
  Yellow = 0b011,
  Green = 0b001,
  Cyan = 0b101,
  Blue = 0b100,
  Magenta = 0b110,
  Black = 0b000,
  White = 0b111,
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
    *super::PD_CFG_REG[2] &= 0xfff000ff;
    *super::PD_CFG_REG[2] |= 0x00011100;
  }

  unsafe {
    *super::PD_DATA_REG &= 0b11111111_11100011_11111111_11111111;
    *super::PD_DATA_REG |= (color as u32) << 18;
  }
}
