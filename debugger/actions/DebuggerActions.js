import { socket } from '../socket';

export const REQUEST_PROGRAM = 'requestProgram';
export const REQUEST_NEXT_OPERATION = 'requestNextOperation';
export const LOAD_PROGRAM = 'loadProgram';
export const PERFORM_NEXT = 'performNext';

export function requestProgram() {
  return {
    type: REQUEST_PROGRAM,
    socketMessageName: 'REQUEST_PROGRAM',
    socketMessage: {programName: 'follower'}
  };
}

export function requestNextOperation() {
  return {
    type: REQUEST_NEXT_OPERATION,
    socketMessageName: 'REQUEST_NEXT_OPERATION',
    socketMessage: {}
  };
}

export function loadProgram() {
  return {
    type: LOAD_PROGRAM
  };
}

export function performNextOperation() {
  return {
    type: PERFORM_NEXT
  };
}
