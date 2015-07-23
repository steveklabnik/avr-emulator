use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use rustc_serialize::hex::FromHex;

use assembler;
use opcodes;

#[derive(Clone, Debug)]
pub struct AvrDataMemory {
    pub registers: Vec<u8>,
    pub io: Vec<u8>,
    pub ram: Vec<u8>
}

#[derive(Clone, Debug)]
pub struct Emulator<'a> {
    pub data_memory: AvrDataMemory,
    pub program_pointer: usize,
    pub machine_code: assembler::MachineCode<'a>
}

impl<'a> Emulator<'a> {
  fn get_next_instruction(&self) -> &assembler::Instruction {
    &self.machine_code.instructions[self.program_pointer]
  }

  pub fn new(program: &str) -> Emulator {
      Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0; 32],
            io: vec![0; 64],
            ram: vec![]
        },
        program_pointer: 0,
        machine_code: assembler::assemble(program)
     }
  }
}

impl<'a> ToJson for Emulator<'a> {
    fn to_json(&self) -> Json {
        let mut data_memory = BTreeMap::new();
        data_memory.insert("registers".to_string(), self.data_memory.registers.to_json());
        data_memory.insert("io".to_string(), self.data_memory.io.to_json());

        let instructions = self.machine_code.instructions.iter().map(|x| x.raw.to_string()).collect::<Vec<String>>();

        let mut emulator = BTreeMap::new();

        emulator.insert("data_memory".to_string(), data_memory.to_json());
        emulator.insert("program_pointer".to_string(), self.program_pointer.to_json());
        emulator.insert("instructions".to_string(), instructions.to_json());
        Json::Object(emulator)
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
      "inc" => opcodes::inc(&emulator, &instruction.operands[0]),
      "ldi" => opcodes::ldi(&emulator, &instruction.operands[0], &instruction.operands[1]),
      "jmp" => opcodes::jmp(&emulator, &instruction.operands[0]),
      _ => opcodes::add(&emulator, &instruction.operands[0], &instruction.operands[1])
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_step() {
      let program = "add r1,r2\nadd r0,r1";
      let mut emulator = Emulator::new(program);
      emulator.data_memory.registers[0] = 1;
      emulator.data_memory.registers[1] = 2;
      emulator.data_memory.registers[2] = 3;

      emulator = step(&emulator);
      assert_eq!(5, emulator.data_memory.registers[1]);
      emulator = step(&emulator);
      assert_eq!(6, emulator.data_memory.registers[0]);
  }


  #[test]
  fn it_serializes() {
      let program = "add r1,r2\nadd r0,r1";
      let mut emulator = Emulator::new(program);
      emulator.data_memory.registers[0] = 1;
      emulator.data_memory.registers[1] = 2;
      emulator.data_memory.registers[2] = 3;

      assert_eq!("{\"data_memory\":{\"io\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],\"registers\":[1,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},\"instructions\":[\"add r1,r2\",\"add r0,r1\"],\"program_pointer\":0}", serialize(&emulator));
  }
}
