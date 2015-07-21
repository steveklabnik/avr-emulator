use rustc_serialize::json::{ToJson, Json};
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
    emulator.to_json().to_string()
}

pub fn get_register_index(operand: &str) -> usize {
    let index = operand.replace("r", "").parse::<usize>();
    index.unwrap()
}

pub fn hex_to_int(operand: &str) -> u8 {
    let hex = operand.replace("$", "");
    hex.from_hex().unwrap()[0]
}

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


//#[test]
//fn it_serializes() {
    //let emulator = Emulator {
        //data_memory: AvrDataMemory {
            //registers: vec![0,2,3],
            //io: vec![],
            //ram: vec![]
        //},
        //program_pointer: 0,
        //machine_code: assembler::assemble("add r1,r2")
    //};
    //assert_eq!("{\"data_memory\":{\"registers\":[0,2,3],\"io\":[],\"ram\":[]}}", serialize(&emulator));
//}
