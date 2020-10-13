#![feature(const_fn, fixed_size_array, lang_items, min_const_generics, start, test)]
#![no_main]
#![no_std]

extern crate cortex_a;

mod hw;
use hw::registers::Register;

#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
  loop { }
}

#[no_mangle]
unsafe fn _start() -> ! {
  // cfg:  0x01C20874
  // data: 0x01C2087C
  const CYCLES: usize = 50_000;
  const CFG_REG: Register<u32> = unsafe { Register::at_addr(0x01C20874) };
  const DATA_REG: Register<u32> = unsafe { Register::at_addr(0x01C2087C) };

  // core::ptr::write_volatile(0x01C20874 as *mut u32, 0x77711177);
  CFG_REG.write(0x77711177);

  loop {
    for _ in 0..CYCLES {
      // core::ptr::write_volatile(0x01C2087C as *mut u32, 0x00040000);
      DATA_REG.write(0x00040000);
    }
    for _ in 0..CYCLES {
      // core::ptr::write_volatile(0x01C2087C as *mut u32, 0x00000000);
      DATA_REG.write(0x00000000);
    }
  }
}
