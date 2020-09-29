#![feature(lang_items, start, test)]
#![no_main]
#![no_std]

extern crate cortex_a;

mod hw;

use hw::led::Color;

#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
  loop { }
}

#[no_mangle]
fn _start() {
  for _ in 0..5 {
    hw::led::set(Color::Green);
    hw::spin_delay(25);
    hw::led::set(hw::led::Color::Black);
    hw::spin_delay(25);
  }

  for _ in 0..10 {
    hw::led::set(Color::White);
    hw::vibe::set(true);
    hw::spin_delay(500);
    hw::led::set(Color::Black);
    hw::vibe::set(false);
    hw::spin_delay(500);
  }

  loop {
    hw::led::set(hw::led::Color::Red);
    hw::spin_delay(50);
    hw::led::set(hw::led::Color::Black);
    hw::spin_delay(50);
  }
}
