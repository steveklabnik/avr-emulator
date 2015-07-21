use std::collections::HashMap;
use rustc_serialize::json::{self, ToJson, Json};
use rustc_serialize::hex::FromHex;

use assembler;
use opcodes;

pub struct AvrDataMemory {
    pub registers: Vec<u8>,
    pub io: Vec<u8>,
    pub ram: Vec<u8>
}

pub struct Emulator<'a> {
    pub data_memory: AvrDataMemory,
    pub program_pointer: usize,
    pub machine_code: assembler::MachineCode<'a>
}

impl<'a> Emulator<'a> {
  fn get_next_instruction(&self) -> &assembler::Instruction {
    &self.machine_code.instructions[self.program_pointer]
  }
}

impl<'a> ToJson for Emulator<'a> {
    fn to_json(&self) -> Json {
        Json::String(format!("{}+{}", "hi".to_string(), "hi".to_string()))
    }
}

pub fn serialize(emulator: &Emulator) -> String {
    // Serialize using `json::encode`
    emulator.to_json().to_string()
}

pub struct Instruction {
    label: String,
    operation: String,
    operands: Vec<String>
}

pub fn parse_instruction(instruction: String) -> Instruction {
    let instruction_iterator = instruction.split(" ");
    let instruction_vector = instruction_iterator.collect::<Vec<&str>>();

    let operands_iterator = instruction_vector[1].split(",");
    let operands_vector = operands_iterator.map(|x| x.to_string()).collect::<Vec<String>>();

    Instruction {
        label: "".to_string(),
        operation: instruction_vector[0].to_string(),
        operands: operands_vector
    }
}
pub fn get_register_index(operand: &str) -> usize {
    let index = operand.replace("r", "").parse::<usize>();
    index.unwrap()
}

fn hex_to_int(operand: &String) -> u8 {
    let hex = operand.replace("$", "");
    hex.from_hex().unwrap()[0]
}


//pub fn inc<'a>(emulator: &Emulator, rd: &String) -> Emulator<'a> {
    //let rd_index = get_register_index(rd);

    //let registers = &emulator.data_memory.registers;
    //let result = registers[rd_index] + 1;

    //let mut new_registers = registers.to_vec();
    //new_registers[rd_index] = result;

    //let data_memory = &emulator.data_memory;
    //Emulator {
        //data_memory: AvrDataMemory {
            //registers: new_registers,
            //io: data_memory.io.to_vec(),
            //ram: data_memory.ram.to_vec()
        //},
        //machine_code: assembler::assemble("")
    //}
//}

//pub fn ldi<'a>(emulator: &Emulator<'a>, rd: &String, k: &String) -> Emulator<'a> {
    //let rd_index = get_register_index(rd);
    //let k_value = hex_to_int(k);

    //let registers = &emulator.data_memory.registers;
    //let mut new_registers = registers.to_vec();
    //new_registers[rd_index] = k_value;

    //let data_memory = &emulator.data_memory;
    //Emulator {
        //data_memory: AvrDataMemory {
            //registers: new_registers,
            //io: data_memory.io.to_vec(),
            //ram: data_memory.ram.to_vec()
        //},
        //machine_code: assembler::assemble("")
    //}
//}

pub fn step<'a>(emulator: &Emulator<'a>) -> Emulator<'a> {
    let instruction = &emulator.get_next_instruction();
    match instruction.operation {
      "add" => opcodes::add(&emulator, &instruction.operands[0], &instruction.operands[1]),
      //"inc" => inc(&emulator, &instruction.operands[0]),
      //"ldi" => ldi(&emulator, &instruction.operands[0], &instruction.operands[1]),
      _ => opcodes::add(&emulator, &instruction.operands[0], &instruction.operands[1])
    }
}

#[test]
fn can_step() {
    let mut emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![1,2,3],
            io: vec![],
            ram: vec![]
        },
        program_pointer: 0,
        machine_code: assembler::assemble("add r1,r2\nadd r0,r1")
    };
    emulator = step(&emulator);
    assert_eq!(5, emulator.data_memory.registers[1]);
    emulator = step(&emulator);
    assert_eq!(6, emulator.data_memory.registers[0]);
}


#[test]
fn it_serializes() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,2,3],
            io: vec![],
            ram: vec![]
        },
        program_pointer: 0,
        machine_code: assembler::assemble("add r1,r2")
    };
    assert_eq!("{\"data_memory\":{\"registers\":[0,2,3],\"io\":[],\"ram\":[]}}", serialize(&emulator));
}

//#[test]
//fn can_ldi() {
    //let emulator = Emulator {
        //data_memory: AvrDataMemory {
            //registers: vec![1,0,0],
            //io: vec![],
            //ram: vec![]
        //},
        //machine_code: assembler::assemble("")
    //};
    //let instruction_line = "ldi r0,$0F".to_string();
    //let next_emulator = perform_instruction(&emulator, instruction_line);
    //assert_eq!(15, next_emulator.data_memory.registers[0]);
//}

//#[test]
//fn can_inc() {
    //let emulator = Emulator {
        //data_memory: AvrDataMemory {
            //registers: vec![0,2,3],
            //io: vec![],
            //ram: vec![]
        //},
        //machine_code: assembler::assemble("")
    //};
    //let instruction_line = "inc r0".to_string();
    //let next_emulator = perform_instruction(&emulator, instruction_line);
    //assert_eq!(1, next_emulator.data_memory.registers[0]);
//}
