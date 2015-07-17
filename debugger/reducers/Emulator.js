import { LOAD_PROGRAM, PERFORM_NEXT} from '../actions/DebuggerActions';

const initialState = {
  registers: [],
  stack: []
};

export default function emulator(state = initialState, action) {
  switch (action.type) {
    case LOAD_PROGRAM:
      return {
       registers: ['init'],
       stack: [1, 2]
      };
    case PERFORM_NEXT:
      if (action.event.data !== 'Hello') {
        debugger
        var data = JSON.parse(action.event.data)
        state.registers = data.registers;
        state.stack = data.stack;
      }
      return {
       registers: state.registers,
       // This should be state.stack 
       stack: [1,2]
      };
    default:
      return state;
  }
}
