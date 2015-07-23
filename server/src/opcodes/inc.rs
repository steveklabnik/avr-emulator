use avr_bit_vec;
use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;

pub fn perform<'a>(emulator: &Emulator<'a>, rd: &str) -> Emulator<'a> {
    let rd_index = get_register_index(rd);

    ///////////////////////////////////////////////////
    // Update Rd //////////////////////////////////////
    ///////////////////////////////////////////////////
    let registers = &emulator.data_memory.registers;
    let result = (registers[rd_index] as u16) + 1;

    let mut new_registers = registers.to_vec();
    let register_result = result as u8;
    new_registers[rd_index] = register_result;

    ///////////////////////////////////////////////////
    // Update SREG flags //////////////////////////////
    ///////////////////////////////////////////////////
    let io = &emulator.data_memory.io;
    let mut new_io = emulator.data_memory.io.to_vec();
    let mut sreg = avr_bit_vec::from_u8(io[63]);
    let r_vec = avr_bit_vec::from_u8(register_result);

    // SREG Z
    sreg.set(1, (register_result == 0b00000000));

    // SREG N
    sreg.set(2, r_vec[7]);

    // SREG V
    sreg.set(3, (register_result == 0b10000000));

    // SREG S
    let n_xor_v = sreg[2] ^ sreg[3];
    sreg.set(4, n_xor_v);

    new_io[63] = avr_bit_vec::to_u8(sreg);

    ///////////////////////////////////////////////////
    // Build and return new emulator state ////////////
    ///////////////////////////////////////////////////
    let data_memory = &emulator.data_memory;
    Emulator {
        data_memory: AvrDataMemory {
            registers: new_registers,
            io: new_io,
            ram: data_memory.ram.to_vec()
        },
        program_pointer: &emulator.program_pointer+1,
        machine_code: emulator.machine_code.clone()
    }
}

#[cfg(test)]
mod tests {
  use avr_bit_vec;
  use emulator::Emulator;

  use super::*;

  #[test]
  fn can_inc() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 2;
      emulator = perform(&emulator, "r1");
      assert_eq!(3, emulator.data_memory.registers[1]);
      emulator = perform(&emulator, "r1");
      assert_eq!(4, emulator.data_memory.registers[1]);

      emulator.data_memory.registers[1] = 255;
      emulator = perform(&emulator, "r1");
      assert_eq!(0, emulator.data_memory.registers[1]);
  }

  #[test]
  fn updates_program_pointer() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator = perform(&emulator, "r1");
      assert_eq!(1, emulator.program_pointer);
      emulator = perform(&emulator, "r1");
      assert_eq!(2, emulator.program_pointer);
  }

  #[test]
  fn updates_sreg_z() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 254;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[1]);

      emulator.data_memory.registers[1] = 255;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[1]);

      emulator.data_memory.registers[1] = 0;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[1]);
  }

  #[test]
  fn updates_sreg_n() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 126;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);

      emulator.data_memory.registers[1] = 127;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);

      emulator.data_memory.registers[1] = 254;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);

      emulator.data_memory.registers[1] = 255;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);
  }

  #[test]
  fn updates_sreg_v() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 126;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[3]);

      emulator.data_memory.registers[1] = 127;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[3]);

      emulator.data_memory.registers[1] = 128;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[3]);
  }

  #[test]
  fn updates_sreg_s() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 126;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);
      assert_eq!(false, sreg[3]);
      assert_eq!(false, sreg[4]);

      emulator.data_memory.registers[1] = 127;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);
      assert_eq!(true, sreg[3]);
      assert_eq!(false, sreg[4]);

      emulator.data_memory.registers[1] = 128;
      emulator = perform(&emulator, "r1");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);
      assert_eq!(false, sreg[3]);
      assert_eq!(true, sreg[4]);
    }
}
