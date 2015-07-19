use std::collections::HashMap;
use rustc_serialize::json;
use rustc_serialize::hex::FromHex;

use opcodes::add;

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
pub fn get_register_index(operand: &String) -> usize {
    let index = operand.replace("r", "").parse::<usize>();
    index.unwrap()
}

fn hex_to_int(operand: &String) -> u8 {
    let hex = operand.replace("$", "");
    hex.from_hex().unwrap()[0]
}


pub fn inc(emulator: &Emulator, rd: &String) -> Emulator {
    let rd_index = get_register_index(rd);

    let registers = &emulator.data_memory.registers;
    let result = registers[rd_index] + 1;

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

pub fn ldi(emulator: &Emulator, rd: &String, k: &String) -> Emulator {
    let rd_index = get_register_index(rd);
    let k_value = hex_to_int(k);

    let registers = &emulator.data_memory.registers;
    let mut new_registers = registers.to_vec();
    new_registers[rd_index] = k_value;

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
    match &instruction.operation.to_owned()[..] { // http://stackoverflow.com/a/23977218
      "add" => add::perform(&emulator, &instruction.operands[0], &instruction.operands[1]),
      "inc" => inc(&emulator, &instruction.operands[0]),
      "ldi" => ldi(&emulator, &instruction.operands[0], &instruction.operands[1]),
      _ => inc(&emulator, &instruction.operands[0]),
    }
}

#[test]
fn can_ldi() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![1,0,0],
            io: vec![],
            ram: vec![]
        }
    };
    let instruction_line = "ldi r0,$0F".to_string();
    let next_emulator = perform_instruction(&emulator, instruction_line);
    assert_eq!(15, next_emulator.data_memory.registers[0]);
}

#[test]
fn can_inc() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,2,3],
            io: vec![],
            ram: vec![]
        }
    };
    let instruction_line = "inc r0".to_string();
    let next_emulator = perform_instruction(&emulator, instruction_line);
    assert_eq!(1, next_emulator.data_memory.registers[0]);
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
