import { WEBSOCKET_UPDATE } from '../actions/DebuggerActions';

export const socket = new WebSocket("ws://72.2.118.4:8000", "rust-websocket");

export function initializeSocketListener(store) {
  socket.onmessage = function (event) {
    store.dispatch({
      type: WEBSOCKET_UPDATE,
      event: event
    });
  };
}
