use avr_bit_vec;
use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;

pub fn perform<'a>(emulator: &Emulator<'a>, rd: &str, rr: &str) -> Emulator<'a> {
    let rd_index = get_register_index(rd);
    let rr_index = get_register_index(rr);

    ///////////////////////////////////////////////////
    // Update Rd //////////////////////////////////////
    ///////////////////////////////////////////////////
    let registers = &emulator.data_memory.registers;
    let result = (registers[rd_index] as u16) + (registers[rr_index] as u16);

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
    let rd_vec = avr_bit_vec::from_u8(registers[rd_index]);
    let rr_vec = avr_bit_vec::from_u8(registers[rr_index]);

    // SREG C
    // This is the same as:
    // let carry = result >= 256
    let carry =
      (rd_vec[7] && rr_vec[7]) ||
      (rr_vec[7] && !r_vec[7]) ||
      (rd_vec[7] && !r_vec[7]);
    sreg.set(0, carry);

    // SREG Z
    sreg.set(1, (register_result == 0b00000000));

    // SREG N
    sreg.set(2, r_vec[7]);

    // SREG V
    let twos_complement_overflow =
      (rd_vec[7] && rr_vec[7] && !r_vec[7]) ||
      (!rd_vec[7] && !rr_vec[7] && r_vec[7]);
    sreg.set(3, twos_complement_overflow);

    // SREG S
    let n_xor_v = sreg[2] ^ sreg[3];
    sreg.set(4, n_xor_v);

    // SREG H
    let mid_carry =
      (rd_vec[3] && rr_vec[3]) ||
      (rr_vec[3] && !r_vec[3]) ||
      (rd_vec[3] && !r_vec[3]);
    sreg.set(5, mid_carry);

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
  fn can_add() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);
      emulator.data_memory.registers[1] = 2;
      emulator.data_memory.registers[2] = 3;

      emulator = perform(&emulator, "r1", "r2");
      assert_eq!(5, emulator.data_memory.registers[1]);
      assert_eq!(3, emulator.data_memory.registers[2]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      assert_eq!(0, emulator.data_memory.registers[1]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 2;
      emulator = perform(&emulator, "r1", "r2");
      assert_eq!(1, emulator.data_memory.registers[1]);
  }

  #[test]
  fn updates_program_pointer() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator = perform(&emulator, "r1", "r2");
      assert_eq!(1, emulator.program_pointer);
      emulator = perform(&emulator, "r1", "r2");
      assert_eq!(2, emulator.program_pointer);
  }

  #[test]
  fn updates_sreg_c() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 128;
      emulator.data_memory.registers[2] = 127;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[0]);

      emulator.data_memory.registers[1] = 128;
      emulator.data_memory.registers[2] = 128;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[0]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 255;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[0]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[0]);

      emulator.data_memory.registers[1] = 1;
      emulator.data_memory.registers[2] = 255;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[0]);
  }

  #[test]
  fn updates_sreg_z() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 0;
      emulator.data_memory.registers[2] = 0;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[1]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[1]);

      emulator.data_memory.registers[1] = 1;
      emulator.data_memory.registers[2] = 0;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[1]);
  }

  #[test]
  fn updates_sreg_n() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 126;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);

      emulator.data_memory.registers[1] = 127;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);

      emulator.data_memory.registers[1] = 254;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 128;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 129;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 255;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);
  }

  #[test]
  fn updates_sreg_v() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 64;
      emulator.data_memory.registers[2] = 63;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[3]);

      emulator.data_memory.registers[1] = 64;
      emulator.data_memory.registers[2] = 64;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[3]);

      emulator.data_memory.registers[1] = 127;
      emulator.data_memory.registers[2] = 127;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[3]);

      emulator.data_memory.registers[1] = 127;
      emulator.data_memory.registers[2] = 128;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[3]);

      emulator.data_memory.registers[1] = 128;
      emulator.data_memory.registers[2] = 127;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[3]);

      emulator.data_memory.registers[1] = 128;
      emulator.data_memory.registers[2] = 128;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[3]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 255;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[3]);
  }

  #[test]
  fn updates_sreg_s() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 0;
      emulator.data_memory.registers[2] = 0;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);
      assert_eq!(false, sreg[3]);
      assert_eq!(false, sreg[4]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 255;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);
      assert_eq!(false, sreg[3]);
      assert_eq!(true, sreg[4]);

      emulator.data_memory.registers[1] = 255;
      emulator.data_memory.registers[2] = 128;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[2]);
      assert_eq!(true, sreg[3]);
      assert_eq!(true, sreg[4]);

      emulator.data_memory.registers[1] = 127;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[2]);
      assert_eq!(true, sreg[3]);
      assert_eq!(false, sreg[4]);
    }

  #[test]
  fn updates_sreg_h() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 8;
      emulator.data_memory.registers[2] = 7;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(false, sreg[5]);

      emulator.data_memory.registers[1] = 15;
      emulator.data_memory.registers[2] = 15;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[5]);

      emulator.data_memory.registers[1] = 15;
      emulator.data_memory.registers[2] = 1;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[5]);

      emulator.data_memory.registers[1] = 1;
      emulator.data_memory.registers[2] = 15;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[5]);

      emulator.data_memory.registers[1] = 24;
      emulator.data_memory.registers[2] = 8;
      emulator = perform(&emulator, "r1", "r2");
      let sreg = avr_bit_vec::from_u8(emulator.data_memory.io[63]);
      assert_eq!(true, sreg[5]);
  }
}
