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
        var data = JSON.parse(action.event.data).data_memory;
      return {
       registers: data.registers,
       // This should be state.stack
       stack: data.io
      };
    default:
      return state;
  }
}
