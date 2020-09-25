#![feature(lang_items, start, test)]
#![no_main]
#![no_std]

extern crate cortex_a;

#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
  loop { }
}

#[no_mangle]
fn _start() {
  unsafe {
    *(0x1c20874 as *mut u32) = 0x77711177;
    // cycle colors forever
    loop {
      for color in 0..8 {
        *(0x1c2087c as *mut u32) = color << 17; // 0x000e0000
        for dum in 0..(1000 * 1000 * 500) {
          core::hint::black_box(dum);
        }
      }
    }
  }
}
