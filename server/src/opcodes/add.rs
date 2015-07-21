use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;
//use emulator::perform_instruction;

use assembler;

pub fn perform<'a>(emulator: &Emulator<'a>, rd: &str, rr: &str) -> Emulator<'a> {
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
        },
        machine_code: assembler::assemble("")
    }
}

//#[test]
//fn can_add() {
    //let emulator = Emulator {
        //data_memory: AvrDataMemory {
            //registers: vec![0,2,3],
            //io: vec![],
            //ram: vec![]
        //},
        //machine_code: assembler::assemble("")
    //};
    //let instruction_line = "add r0,r2".to_string();
    //let next_emulator = perform_instruction(&emulator, instruction_line);
    //assert_eq!(3, next_emulator.data_memory.registers[0]);
    //assert_eq!(3, next_emulator.data_memory.registers[2]);
//}
