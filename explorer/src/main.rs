#![feature(const_fn, fixed_size_array, lang_items, min_const_generics, start, test)]
#![no_main]
#![no_std]

mod hw;
use hw::*;

#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
  loop { }
}

#[no_mangle]
extern "C" fn k_main() -> ! {
  const CYCLES: usize = 50_000;

  led::init();

  loop {
    led::set(led::Color::White);
    spin_delay(CYCLES);
    led::set(led::Color::Blue);
    spin_delay(CYCLES);
  }
}
