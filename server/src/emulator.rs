use std::collections::HashMap;
use rustc_serialize::json;


#[derive(RustcDecodable, RustcEncodable)]
pub struct AvrDataMemory {
    pub registers: Vec<u8>,
    pub io: Vec<u8>,
    pub ram: Vec<u8>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Emulator {
    pub data_memory: AvrDataMemory
}

pub fn serialize(emulator: &Emulator) -> String {
    // Serialize using `json::encode`
    json::encode(emulator).unwrap()
}


pub struct Instruction {
    label: String,
    operation: String,
    operands: Vec<String>
}

pub fn parse_instruction(instruction: String) -> Instruction {
    Instruction {
        label: "".to_string(),
        operation: "add".to_string(),
        operands: vec!["r1".to_string(),"r2".to_string()]
    }
}
fn get_register_index(operand: &String) -> usize {
    let index = operand.replace("r", "").parse::<usize>();
    index.unwrap()
}
pub fn add(emulator: &Emulator, rd: &String, rr: &String) -> Emulator {
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
        }
    }
}

// This is a reducer just like redux!
// (emulator, instruction) => (emulator)
// (state, action) => (state)
pub fn perform_instruction(emulator: &Emulator, instruction_line: String) -> Emulator {
    println!("instruction line: {}", instruction_line);
    let instruction = parse_instruction(instruction_line);
    add(&emulator, &instruction.operands[0], &instruction.operands[1])
}

#[test]
fn can_add() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,2,3],
            io: vec![],
            ram: vec![]
        }
    };
    let instruction_line = "add r1,r2".to_string();
    let next_emulator = perform_instruction(&emulator, instruction_line);
    assert_eq!(5, next_emulator.data_memory.registers[1]);
    assert_eq!(3, next_emulator.data_memory.registers[2]);
}

#[test]
fn it_works() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,2,3],
            io: vec![],
            ram: vec![]
        }
    };
    assert_eq!("{\"data_memory\":{\"registers\":[0,2,3],\"io\":[],\"ram\":[]}}", serialize(&emulator));
}
