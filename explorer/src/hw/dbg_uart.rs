// TODO: struct Uart to be generic, etc.
// TODO: Abstract out GPIO config logic since it's the same on all ports

use super::*;

pub fn init(_div: u32) {
  if UART0_SCH.read() & 0xffu32 != 0 { return; }
  UART0_SCH.write(1);

  //TODO: the rest of this, idk
}
