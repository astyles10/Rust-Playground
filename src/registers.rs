// pub mod Registers;

pub struct RegisterPair {
  first: u8,
  second: u8,
  both: u16
}

pub struct Registers {
  af: RegisterPair,
  bc: RegisterPair,
  de: RegisterPair,
  hl: RegisterPair
}

impl Registers {
  // Public
  pub fn init_registers() -> Registers {
    Registers {
      af: RegisterPair { first: 0, second: 0, both: 0 },
      bc: RegisterPair { first: 0, second: 0, both: 0 },
      de: RegisterPair { first: 0, second: 0, both: 0 },
      hl: RegisterPair { first: 0, second: 0, both: 0 }
    }
  }

  pub fn set_single(&mut self, in_value: u8, in_register: char) {
    match in_register {
      'a' => self.update_registers(&mut self.af, in_value.into()),
      'b' => self.update_registers(&mut self.bc, in_value.into()),
      _ => println!("Invalid register: {}", in_register)
    }
  }

  pub fn update_pair(&self, in_value: u16, in_register_pair: String) {
    match in_register_pair.as_str() {
      "af" => self.update_registers(&mut self.af, in_value),
    _ => println!("Invalid register pair: {}", in_register_pair)
    }
  }

  pub fn get_single(&self, in_register: char) -> u8 {
    match in_register {
      'a' => self.af.first,
      'f' => self.af.second,
      'b' => self.bc.first,
      'c' => self.bc.second,
      'd' => self.de.first,
      'e' => self.de.second,
      'h' => self.hl.first,
      'l' => self.hl.second,
      _ => {
        println!("Invalid register: {}", in_register);
        return 0;
      }
    }
  }

  fn update_registers(&self, in_reg_pair: &mut RegisterPair, in_value: u16) {

  }
}
