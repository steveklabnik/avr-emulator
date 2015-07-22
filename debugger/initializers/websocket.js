import { WEBSOCKET_UPDATE } from '../actions/DebuggerActions';

export const socket = new WebSocket("ws://127.0.0.1:2794", "rust-websocket");

export function initializeSocketListener(store) {
  socket.onmessage = function (event) {
    store.dispatch({
      type: WEBSOCKET_UPDATE,
      event: event
    });
  };
}
