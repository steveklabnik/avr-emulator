use emulator::Emulator;

pub fn perform<'a>(emulator: &Emulator<'a>) -> Emulator<'a> {
    ///////////////////////////////////////////////////
    // Build and return new emulator state ////////////
    ///////////////////////////////////////////////////
    Emulator {
        data_memory: emulator.data_memory.clone(),
        program_pointer: &emulator.program_pointer+1,
        machine_code: emulator.machine_code.clone()
    }
}

#[cfg(test)]
mod tests {
  use emulator::Emulator;

  use super::*;

  #[test]
  fn can_nop() {
      let program = "add r0,r0";
      let mut emulator = Emulator::new(program);

      emulator = perform(&emulator);
      assert_eq!(1, emulator.program_pointer);

      emulator = perform(&emulator);
      assert_eq!(2, emulator.program_pointer);
  }
}
