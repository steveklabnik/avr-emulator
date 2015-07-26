import { socket } from '../initializers/websocket';

export const WEBSOCKET_REQUEST = 'websocketRequest';
export const WEBSOCKET_UPDATE = 'websocketUpdate';

export const PERFORM_STEP = 'performStep';
export const EXECUTE_PROGRAM = 'executeProgram';
export const STOP_EXECUTION = 'stopExecution';

export function performStep() {
  return {
    type: WEBSOCKET_REQUEST,
    socketMessageName: PERFORM_STEP,
    socketMessage: {}
  };
}

export function initializeExecution(executionId) {
  return {
    type: EXECUTE_PROGRAM,
    executionId: executionId
  }
}

export function executeProgram() {
  return dispatch => {
    var executionId = setInterval(() => {
      dispatch(performStep());
    }, 250);

    dispatch(initializeExecution(executionId));
  };
}

export function stopExecution(executionId) {
  clearInterval(executionId);
  return {
    type: STOP_EXECUTION,
    executionId: executionId,
  };
}
