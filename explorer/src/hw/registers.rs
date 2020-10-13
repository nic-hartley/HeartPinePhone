// this one's gonna get big...
// TODO: Organize
// - Structs/macros to hold/define common patterns
// - Base + offset? (write out the math from the manual instead of pre-doing it)
// NOTE: DO NOT want to make this too complex
// should be basically just holding lots of numbers
// no business logic in here
// TODO: More efficient ways?
// - *mut [u32; 4] instead of [*mut u32; 4]? syntax was ugly last time but doable?
// - `repr(C)` structs?
// TODO: Ensure all reads/writes are volatile.
// - Wrapper struct around pointers?

#[repr(transparent)]
pub struct Register<Data: Copy>(*mut Data);

impl<Data: Copy> Register<Data> {
  #[inline(always)]
  pub const unsafe fn at_addr(addr: usize) -> Register<Data> {
    Register(addr as *mut Data)
  }

  #[inline(always)]
  pub const unsafe fn at_ptr(ptr: *mut Data) -> Register<Data> {
    Register(ptr)
  }

  #[inline(always)]
  pub fn read(&self) -> Data {
    unsafe { core::ptr::read_volatile(self.0) }
  }

  #[inline(always)]
  pub fn write(&self, new: Data) {
    unsafe { core::ptr::write_volatile(self.0, new) }
  }

  #[inline(always)]
  pub fn mutate<F>(&self, updater: F) -> Data
      where F: FnOnce(&mut Data)
  {
    let mut val = self.read();
    updater(&mut val);
    self.write(val);
    val
  }

  #[inline(always)]
  pub fn update<F>(&self, updater: F) -> Data
      where F: FnOnce(Data) -> Data
  {
    let val = self.read();
    let new_val = updater(val);
    self.write(new_val);
    new_val
  }
}

impl<Elem: Copy, const N: usize> Register<[Elem; N]> {
  pub fn index(&self, index: isize) -> Register<Elem>
  {
    let first_ele = self.0 as *mut Elem;
    unsafe { Register::at_ptr(first_ele.offset(index)) }
  }
}

pub const VER_REG: Register<u32> = unsafe { Register::at_addr(0x01C00024) };

pub const PB_CFG_REG: Register<[u32; 2]> = unsafe { Register::at_addr(0x01C20800) };
pub const PB_DATA_REG: Register<u32> = unsafe { Register::at_addr(0x01C20810) };

pub const PD_CFG_REG: Register<[u32; 4]> = unsafe { Register::at_addr(0x01C2086C) };
pub const PD_DATA_REG: Register<u32> = unsafe { Register::at_addr(0x01C2087C) };

pub const UART0_DAT: Register<u32> = unsafe { Register::at_addr(0x01C28000) };
pub const UART0_FCR: Register<u32> = unsafe { Register::at_addr(0x01C28008) };
pub const UART0_LCR: Register<u32> = unsafe { Register::at_addr(0x01C2800C) };
pub const UART0_SCH: Register<u32> = unsafe { Register::at_addr(0x01C2801C) };
pub const UART0_USR: Register<u32> = unsafe { Register::at_addr(0x01C2807C) };
pub const UART0_TFL: Register<u32> = unsafe { Register::at_addr(0x01C28080) };
pub const UART0_RFL: Register<u32> = unsafe { Register::at_addr(0x01C28084) };

pub const PL_CFG_REG: Register<[u32; 4]> = unsafe { Register::at_addr(0x01F02C00) };
