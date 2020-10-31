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
  led::init();

  for _ in 0..5 {
    for color in [
      led::Color::White,
      led::Color::Yellow,
      led::Color::Cyan,
      led::Color::Magenta,
      led::Color::Red,
      led::Color::Green,
      led::Color::Blue,
      led::Color::Black,
    ] {
      spin_delay(25);
      led::set(color);
    }
  }

  spin_delay(100);
  
}
