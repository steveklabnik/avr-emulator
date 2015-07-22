import { LOAD_PROGRAM, PERFORM_NEXT} from '../actions/DebuggerActions';

const initialState = {
  registers: [],
  instructions: [],
  stack: [],
  program_pointer: 0
};

export default function emulator(state = initialState, action) {
  switch (action.type) {
    case LOAD_PROGRAM:
      return {
        registers: ['init'],
        instructions: [],
        stack: [1, 2],
        program_pointer: 0
      };
    case PERFORM_NEXT:
        var data = JSON.parse(action.event.data);
      return {
        registers: data.data_memory.registers,
        instructions: data.instructions,
        program_pointer: data.program_pointer,
        // This should be state.stack
        stack: data.data_memory.io
      };
    default:
      return state;
  }
}
