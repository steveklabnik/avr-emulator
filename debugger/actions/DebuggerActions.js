import { socket } from '../initializers/websocket';

export const WEBSOCKET_REQUEST = 'websocketRequest';
export const WEBSOCKET_UPDATE = 'websocketUpdate';

export const PERFORM_STEP = 'performStep';

export function performStep() {
  return {
    type: WEBSOCKET_REQUEST,
    socketMessageName: PERFORM_STEP,
    socketMessage: {}
  };
}
