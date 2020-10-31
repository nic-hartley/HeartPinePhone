// any given version of explorer might not use any given part of the HAL
// doesn't mean we hate it and want it gone
// so allow(dead_code) for the whole module
#![allow(dead_code)]
// https://github.com/rust-lang/rust/issues/77321
#![allow(const_item_mutation)]

mod registers;
use registers::*;
pub mod led;
pub mod vibe;
pub mod power;
pub mod dbg_uart;

fn ensure_side_effect() {
  let _ = VER_REG.read();
}

pub fn spin_delay(approx_msec: usize) {
  for _ in 0..approx_msec {
    for _ in 0..1000 {
      ensure_side_effect()
    }
  }
}
