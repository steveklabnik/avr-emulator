use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MachineCode<'a> {
    pub instructions: Vec<Instruction<'a>>,
    pub label_locations: HashMap<&'a str, usize>
}

#[derive(Clone, Debug)]
pub struct Instruction<'a> {
    pub raw: &'a str,
    pub label: &'a str,
    pub operation: &'a str,
    pub operands: Vec<&'a str>
}

pub fn parse_instruction<'a>(raw: &'a str) -> Instruction<'a> {
    // Split line into a vector by ":"
    let raw = raw.trim();
    let line = raw.split(":");
    let mut line = line.collect::<Vec<&str>>();

    // Pop everything to the right of ":"
    // into instruction vector
    let instruction = line.pop().unwrap().trim();
    let mut instruction = instruction.split(" ").collect::<Vec<&str>>();

    // If there's anything left, it's the label
    let label = line.pop().unwrap_or("");

    // Remove the operation from the instruction vector
    let operation = instruction.remove(0);

    // If there's anything left, it's the operands string
    let operands = instruction.pop().unwrap_or("");

    // Create an empty vector if the operands string is empty
    // Fill it if the operands string is not empty
    let mut operands_vector = vec![""; 0];
    if operands != "" {
      operands_vector = operands.split(",").collect::<Vec<&str>>();
    }

    Instruction {
        raw: raw,
        label: label,
        operation: operation,
        operands: operands_vector
    }
}

pub fn assemble<'a>(program: &'a str) -> MachineCode<'a> {
    let instruction_iterator = program.split("\n");
    let mut instructions: Vec<Instruction> = Vec::with_capacity(2);
    let mut label_locations = HashMap::new();

    for (index, line) in instruction_iterator.enumerate() {
        let instruction = parse_instruction(line);
        if instruction.label != "" {
          label_locations.insert(instruction.label, index);
        }

        if instruction.raw != "" {
          instructions.push(instruction);
        }

    }
    MachineCode {
      instructions: instructions,
      label_locations: label_locations
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_assemble_op_no_args() {
      let program = "nop";
      let machine_code = assemble(program);

      let instruction = &machine_code.instructions[0];
      assert_eq!("", instruction.label);
      assert_eq!("nop", instruction.operation);
      assert_eq!(0, instruction.operands.len());
  }

  #[test]
  fn can_assemble_labeled_op_no_args() {
      let program = "redo: nop";
      let machine_code = assemble(program);

      let instruction = &machine_code.instructions[0];
      assert_eq!("redo", instruction.label);
      assert_eq!("nop", instruction.operation);
      assert_eq!(0, instruction.operands.len());
  }

  #[test]
  fn can_assemble_op_single_arg() {
      let program = "inc r0";
      let machine_code = assemble(program);

      let instruction = &machine_code.instructions[0];
      assert_eq!("", instruction.label);
      assert_eq!("inc", instruction.operation);
      assert_eq!(vec!["r0"], instruction.operands);
  }
  #[test]
  fn can_assemble_labeled_op_single_arg() {
      let program = "redo: inc r0";
      let machine_code = assemble(program);

      let instruction = &machine_code.instructions[0];
      assert_eq!("redo", instruction.label);
      assert_eq!("inc", instruction.operation);
      assert_eq!(vec!["r0"], instruction.operands);
  }

  #[test]
  fn can_assemble_op_dual_arg() {
      let program = "add r1,r2";
      let machine_code = assemble(program);

      let instruction = &machine_code.instructions[0];
      assert_eq!("", instruction.label);
      assert_eq!("add", instruction.operation);
      assert_eq!(vec!["r1", "r2"], instruction.operands);
  }
  #[test]
  fn can_assemble_labeled_op_dual_arg() {
      let program = "redo: add r1,r2";
      let machine_code = assemble(program);

      let instruction = &machine_code.instructions[0];
      assert_eq!("redo", instruction.label);
      assert_eq!("add", instruction.operation);
      assert_eq!(vec!["r1", "r2"], instruction.operands);
  }

  #[test]
  fn ignores_leading_and_trailing_whitespace() {
      let program = " redo: add r1,r2 ";
      let machine_code = assemble(program);

      let instruction = &machine_code.instructions[0];
      assert_eq!("redo", instruction.label);
      assert_eq!("add", instruction.operation);
      assert_eq!(vec!["r1", "r2"], instruction.operands);
  }

  #[test]
  fn skips_empty_lines() {
      let program = " ";
      let machine_code = assemble(program);

      assert_eq!(machine_code.instructions.len(), 0);
  }

  #[test]
  fn can_track_label_locations() {
      let program = "ldi r0,$0f\nspecial: inc r0";
      let machine_code = assemble(program);

      assert_eq!(machine_code.instructions.len(), 2);

      let labels = &machine_code.label_locations;
      assert_eq!(labels.get("special"), Some(&1usize));
  }
}
