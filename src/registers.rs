// pub mod Registers;
use object::{endian, LittleEndian, BigEndian, Endianness, Endian};

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
    // let endian: LittleEndian = endian::NativeEndian;
    // match endian {
    //   LittleEndian => {

    //   },
    //   BigEndian => {

    //   }
    // }

    Registers {
      af: RegisterPair { first: 0, second: 0, both: 0 },
      bc: RegisterPair { first: 0, second: 0, both: 0 },
      de: RegisterPair { first: 0, second: 0, both: 0 },
      hl: RegisterPair { first: 0, second: 0, both: 0 }
    }
  }

  pub fn set_single(&mut self, in_value: u8, in_register: char) {
    match in_register {
      'a' => update_registers(&mut self.af, in_value.into()),
      'b' => update_registers(&mut self.bc, in_value.into()),
      _ => println!("Invalid register: {}", in_register)
    }
  }

  pub fn set_pair(&mut self, in_value: u16, in_register_pair: String) {
    match in_register_pair.as_str() {
      "af" => update_registers(&mut self.af, in_value),
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
}

fn update_registers(in_reg_pair: &mut RegisterPair, in_value: u16) {

}

pub fn determine_endianess() -> Endianness {
  use std::mem;
  let i: u16 = 0x1234;
  let mut _bs = [0u8; mem::size_of::<u16>()];
  _bs = unsafe { mem::transmute(i) };
  if _bs[0] == 0x12 {
    return Endianness::Big;
  }
  return Endianness::Little;
}
