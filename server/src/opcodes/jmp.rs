use emulator::Emulator;
use emulator::AvrDataMemory;
use emulator::get_register_index;

use assembler;

pub fn perform<'a>(emulator: &Emulator<'a>, k: &str) -> Emulator<'a> {
    let label = k;

    let label_locations = &emulator.machine_code.label_locations;
    let program_pointer = label_locations.get(label).unwrap();

    Emulator {
        data_memory: emulator.data_memory.clone(),
        program_pointer: *program_pointer,
        machine_code: emulator.machine_code.clone()
    }
}

#[test]
fn can_jmp() {
    let program = "add r0,r0\nadd r0,r0\nadd r0,r0\nspecial inc r0";
    let emulator = Emulator {
        data_memory: AvrDataMemory {
            registers: vec![0,2,3],
            io: vec![],
            ram: vec![]
        },
        program_pointer: 0,
        machine_code: assembler::assemble(program)
    };
    let emulator = perform(&emulator, "special");
    assert_eq!(3, emulator.program_pointer);
}
