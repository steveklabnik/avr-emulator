use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;

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
        program_pointer: &emulator.program_pointer+1,
        machine_code: emulator.machine_code.clone()
    }
}

#[test]
fn can_add() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,2,3],
            io: vec![],
            ram: vec![]
        },
        program_pointer: 0,
        machine_code: assembler::assemble("add r0,r0")
    };
    let next_emulator = perform(&emulator, "r1", "r2");
    assert_eq!(5, next_emulator.data_memory.registers[1]);
    assert_eq!(3, next_emulator.data_memory.registers[2]);
}

#[test]
fn updates_program_pointer() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,2,3],
            io: vec![],
            ram: vec![]
        },
        program_pointer: 0,
        machine_code: assembler::assemble("add r0,r0")
    };
    let next_emulator = perform(&emulator, "r1", "r2");
    assert_eq!(1, next_emulator.program_pointer);
}
