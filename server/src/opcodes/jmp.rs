use emulator::Emulator;

pub fn perform<'a>(emulator: &Emulator<'a>, k: &str) -> Emulator<'a> {
    let label = k;

    let label_locations = &emulator.machine_code.label_locations;

    let program_pointer = match label_locations.get(label) {
        None => panic!("Unknown instruction label"),
        Some(&p) => p
    };

    Emulator {
        data_memory: emulator.data_memory.clone(),
        program_pointer: program_pointer,
        machine_code: emulator.machine_code.clone()
    }
}

#[cfg(test)]
mod tests {
  use emulator::Emulator;

  use super::*;

  #[test]
  fn can_jmp() {
      let program = "add r0,r0\nadd r0,r0\nadd r0,r0\nspecial inc r0";
      let mut emulator = Emulator::new(program);
      emulator = perform(&emulator, "special");
      assert_eq!(3, emulator.program_pointer);
  }

  #[test]
  #[should_panic(expected = "Unknown instruction label")]
  fn handles_invalid_label() {
      let program = "add r0,r0\nadd r0,r0\nadd r0,r0\nspecial inc r0";
      let emulator = Emulator::new(program);
      perform(&emulator, "bogus");
  }
}
