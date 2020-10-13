// any given version of explorer might not use any given part of the HAL
// doesn't mean we hate it and want it gone
// so allow(dead_code) for the whole module
#![allow(dead_code)]
// https://github.com/rust-lang/rust/issues/77321
#![allow(const_item_mutation)]

pub mod registers;
use registers::*;
pub mod led;
pub mod vibe;
pub mod power;
pub mod dbg_uart;

#[inline(always)]
pub fn ensure_side_effect() {
  let _ = VER_REG.read();
}

#[inline(always)]
pub fn spin_delay(loops: usize) {
  for _ in 0..loops {
    ensure_side_effect()
  }
}
