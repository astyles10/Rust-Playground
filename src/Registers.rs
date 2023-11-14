pub struct registers {
  pub a: u8,
  pub b: u8
}

impl registers {
  pub fn set_a(&mut self, value: u8) {
    self.a = value;
  }
}