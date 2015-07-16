export const LOAD_PROGRAM = 'loadProgram';
export const PERFORM_NEXT = 'performNext';

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
