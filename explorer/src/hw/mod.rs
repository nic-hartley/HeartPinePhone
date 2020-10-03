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
  let _ = unsafe { core::ptr::read_volatile(VER_REG) };
}

pub fn spin_delay(rough_usecs: usize) {
  for _ in 0..rough_usecs {
    for d in 0..1000 {
      core::hint::black_box(d);
      ensure_side_effect();
    }
  }
}