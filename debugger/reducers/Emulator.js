import { WEBSOCKET_UPDATE, EXECUTE_PROGRAM, STOP_EXECUTION } from '../actions/DebuggerActions';

const initialState = {
  data_memory: {
    registers: [],
    io: []
  },
  instructions: [],
  program_pointer: 0,
  executionId: null
};

export default function emulator(state = initialState, action) {
  console.log('current state: ', state);
  switch (action.type) {
    case WEBSOCKET_UPDATE:
      var response = JSON.parse(action.event.data);
      response.executionId = state.executionId;
      return response
    case EXECUTE_PROGRAM:
      console.log('run in reducer', action, action.executionId);
      return {
        data_memory: state.data_memory,
        instructions: state.instructions,
        program_pointer: state.program_pointer,
        executionId: action.executionId
      };
    case STOP_EXECUTION:
      console.log('stop in reducer', action);
      return {
        data_memory: state.data_memory,
        instructions: state.instructions,
        program_pointer: state.program_pointer,
        executionId: null
      };
    default:
      return state;
  }
}
