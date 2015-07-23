use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;
use emulator::hex_to_int;

pub fn perform<'a>(emulator: &Emulator<'a>, a: &str, rd: &str) -> Emulator<'a> {
    let rd_index = get_register_index(rd);
    let io_address = hex_to_int(a);

    ///////////////////////////////////////////////////
    // Update IO Address //////////////////////////////
    ///////////////////////////////////////////////////

    let registers = &emulator.data_memory.registers;
    let mut new_io = emulator.data_memory.io.to_vec();
    println!("IO Adress: {}", io_address);
    new_io[io_address as usize] = registers[rd_index];

    ///////////////////////////////////////////////////
    // Build and return new emulator state ////////////
    ///////////////////////////////////////////////////
    let data_memory = &emulator.data_memory;
    Emulator {
        data_memory: AvrDataMemory {
            registers: data_memory.registers.to_vec(),
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
  fn can_perform() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 8;
      emulator = perform(&emulator, "$18", "r1");
      let portb = avr_bit_vec::from_u8(emulator.data_memory.io[0x18]);
      assert_eq!(8, avr_bit_vec::to_u8(portb));
  }

  #[test]
  fn updates_program_pointer() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator.data_memory.registers[1] = 8;
      emulator = perform(&emulator, "$18", "r1");
      assert_eq!(1, emulator.program_pointer);
      emulator = perform(&emulator, "$18", "r1");
      assert_eq!(2, emulator.program_pointer);
  }
}
