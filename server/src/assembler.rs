use std::collections::HashMap;

pub struct MachineCode<'a> {
    instructions: Vec<Instruction<'a>>,
    label_locations: HashMap<&'a str, i32>
}

pub struct Instruction<'a> {
    label: &'a str,
    operation: String,
    operands: Vec<String>
}

pub fn parse_instruction<'a>(instruction: String) -> Instruction<'a> {
    let mut instruction_iterator = instruction.split(" ");
    let instruction_vector = instruction_iterator.collect::<Vec<&str>>();

    let mut operands_iterator = instruction_vector[1].split(",");
    let operands_vector = operands_iterator.map(|x| x.to_string()).collect::<Vec<String>>();

    Instruction {
        label: "",
        operation: instruction_vector[0].to_string(),
        operands: operands_vector
    }
}

pub fn assemble<'a>(program: String) -> Vec<Instruction<'a>> {
    let instruction_iterator = program.split("\n");
    let mut instructions: Vec<Instruction> = Vec::with_capacity(2);
    let mut label_locations = HashMap::new();

    for (index, line) in instruction_iterator.enumerate() {
        println!("{}", line);
        let instruction = parse_instruction(line.to_string());
        if (instruction.label != "") {
          label_locations.insert(instruction.label, index);
        }

        instructions.push(instruction);

    }
    instructions
}

#[test]
fn can_assemble() {
    let program = "ldi r0,$0f\ninc r0".to_string();
    let instructions = assemble(program);

    assert_eq!(instructions.len(), 2);

    let ldi = &instructions[0];
    assert_eq!("", ldi.label);
    assert_eq!("ldi", ldi.operation);
    assert_eq!(vec!["r0","$0f"], ldi.operands);

    let inc = &instructions[1];
    assert_eq!("", inc.label);
    assert_eq!("inc", inc.operation);
    assert_eq!(vec!["r0"], inc.operands);
}
