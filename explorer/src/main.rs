#![feature(
  // enable non-Sized trait bound on Register (because its ctors are const)
  const_fn,
  // enable #[panic_handler]
  lang_items,
  // enable specializing Register for arrays, without hacky workarounds
  min_const_generics
)]
#![no_main]
#![no_std]

mod hw;
use hw::*;

#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
  loop {
    led::set(led::Color::Red);
    spin_delay(25);
  }
}

#[no_mangle]
extern "C" fn k_main() -> ! {
  led::init();

  for color in &[
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
    led::set(*color);
  }

  led::set(led::Color::Red);

  dbg_uart::init(0);
  loop {
    led::set(led::Color::Green);
    dbg_uart::write(b"Hello, ");
    led::set(led::Color::Blue);
    dbg_uart::write(b"World!\n");
  }
}
