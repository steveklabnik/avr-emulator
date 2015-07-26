import { WEBSOCKET_UPDATE, EXECUTE_PROGRAM, STOP_EXECUTION } from '../actions/DebuggerActions';

const initialState = {
  data_memory: {
    registers: [],
    io: []
  },
  instructions: [],
  program_pointer: 0
};

export default function emulator(state = initialState, action) {
  switch (action.type) {
    case WEBSOCKET_UPDATE:
      return JSON.parse(action.event.data);
    default:
      return state;
  }
}
