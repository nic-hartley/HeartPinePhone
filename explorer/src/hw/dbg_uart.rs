// TODO: struct Uart to be generic, etc.
// TODO: Abstract out GPIO config logic since it's the same on all ports

use super::*;

pub fn init(_div: u32) {
  unsafe {
    if *UART0_SCH & 0xff != 0 { return; }
    *UART0_SCH = 1;
  }

  //TODO: the rest of this, idk
}
