// TODO: struct Uart to be generic, etc.
// TODO: Abstract out GPIO config logic since it's the same on all ports

use super::*;

pub fn init(_div: u32) {
  // PB_CFG1_REG only has the two entries for the UART0 TX/RX, so blow it away
  PB_CFG_REG.index(1).write(0x44);
  // odd parity, parity enabled, 1 stop bit, 8-bit data
  UART0_LCR.write(0b00001011);
}

pub fn write(data: &[u8]) {
  for b in data {
    UART0_DAT.write(*b as u32);
    // wait for room to write another
    while UART0_LSR.read() & (1 << 5) == 0 { ensure_side_effect(); }
  }
}
