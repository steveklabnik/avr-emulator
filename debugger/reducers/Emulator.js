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
      return {
       registers: [...state.registers, 'hi'],
       stack: [1, 2]
      }
    default:
      return state;
  }
}
