import { EXECUTE_PROGRAM, STOP_EXECUTION } from '../actions/DebuggerActions';

const initialState = {
  executionId: null
};

export default function programRunner(state = initialState, action) {
  switch (action.type) {
    case EXECUTE_PROGRAM:
      return {
        executionId: action.executionId
      };
    case STOP_EXECUTION:
      return {
        executionId: null
      };
    default:
      return state;
  }
}

