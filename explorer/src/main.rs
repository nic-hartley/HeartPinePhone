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
fn _start() -> ! {
  hw::led::set(Color::White);
  loop {}
}
