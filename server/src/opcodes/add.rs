use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;

pub fn perform<'a>(emulator: &Emulator<'a>, rd: &str, rr: &str) -> Emulator<'a> {
    let rd_index = get_register_index(rd);
    let rr_index = get_register_index(rr);

    let registers = &emulator.data_memory.registers;
    let result = registers[rd_index] + registers[rr_index];

    let mut new_registers = registers.to_vec();
    new_registers[rd_index] = result;

    let data_memory = &emulator.data_memory;
    Emulator {
        data_memory: AvrDataMemory {
            registers: new_registers,
            io: data_memory.io.to_vec(),
            ram: data_memory.ram.to_vec()
        },
        program_pointer: &emulator.program_pointer+1,
        machine_code: emulator.machine_code.clone()
    }
}

#[cfg(test)]
mod tests {
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
}
