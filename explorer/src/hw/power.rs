pub fn off() -> ! {
  loop { super::ensure_side_effect(); }
}
