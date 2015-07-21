use std::collections::HashMap;

pub struct MachineCode<'a> {
    pub instructions: Vec<Instruction<'a>>,
    label_locations: HashMap<&'a str, usize>
}

pub struct Instruction<'a> {
    pub label: &'a str,
    pub operation: &'a str,
    pub operands: Vec<&'a str>
}

pub fn parse_instruction<'a>(instruction: &'a str) -> Instruction<'a> {
    let instruction_iterator = instruction.split(" ");
    let mut instruction_vector = instruction_iterator.collect::<Vec<&str>>();

    let operands_iterator = instruction_vector.pop().unwrap().split(",");
    let operands_vector = operands_iterator.collect::<Vec<&str>>();

    let operation = instruction_vector.pop().unwrap();
    let label = instruction_vector.pop().unwrap_or("");

    Instruction {
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
        println!("{}", line);
        let instruction = parse_instruction(line);
        if (instruction.label != "") {
          label_locations.insert(instruction.label, index);
        }

        instructions.push(instruction);

    }
    MachineCode {
      instructions: instructions,
      label_locations: label_locations
    }
}

#[test]
fn can_assemble() {
    let program = "ldi r0,$0f\nspecial inc r0";
    let machine_code = assemble(program);

    assert_eq!(machine_code.instructions.len(), 2);

    let ldi = &machine_code.instructions[0];
    assert_eq!("", ldi.label);
    assert_eq!("ldi", ldi.operation);
    assert_eq!(vec!["r0","$0f"], ldi.operands);

    let inc = &machine_code.instructions[1];
    assert_eq!("special", inc.label);
    assert_eq!("inc", inc.operation);
    assert_eq!(vec!["r0"], inc.operands);

    let labels = &machine_code.label_locations;
    assert_eq!(labels.get("special"), Some(&1usize));
}
