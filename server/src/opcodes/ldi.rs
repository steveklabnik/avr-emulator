use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;
use emulator::hex_to_int;

use assembler;

pub fn perform<'a>(emulator: &Emulator<'a>, rd: &str, k: &str) -> Emulator<'a> {
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
        },
        program_pointer: &emulator.program_pointer+1,
        machine_code: emulator.machine_code.clone()
    }
}

#[test]
fn can_ldi() {
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,0,0],
            io: vec![],
            ram: vec![]
        },
        program_pointer: 0,
        machine_code: assembler::assemble("add r0,r0")
    };
    let emulator = perform(&emulator, "r1", "$0F");
    assert_eq!(15, emulator.data_memory.registers[1]);
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
    let emulator = perform(&emulator, "r1", "$0F");
    assert_eq!(1, emulator.program_pointer);
    let emulator = perform(&emulator, "r1", "$00");
    assert_eq!(2, emulator.program_pointer);
}
