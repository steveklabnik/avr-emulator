pub struct Instruction {
    label: String,
    operation: String,
    operands: Vec<String>
}

pub fn parse_instruction(instruction: String) -> Instruction {
    let mut instruction_iterator = instruction.split(" ");
    let instruction_vector = instruction_iterator.collect::<Vec<&str>>();

    let mut operands_iterator = instruction_vector[1].split(",");
    let operands_vector = operands_iterator.map(|x| x.to_string()).collect::<Vec<String>>();

    Instruction {
        label: "".to_string(),
        operation: instruction_vector[0].to_string(),
        operands: operands_vector
    }
}

pub fn assemble(program: String) -> Vec<Instruction> {
    let mut instruction_iterator = program.split("\n");
    let mut vec: Vec<Instruction> = Vec::with_capacity(2);
    for line in instruction_iterator {
        println!("{}", line);
        vec.push(parse_instruction(line.to_string()));
    }
    vec
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
