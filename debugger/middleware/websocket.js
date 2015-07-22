import { socket } from '../initializers/websocket';

export default function websocketMiddleware({ dispatch, getState }) {
  return (next) => (action) => {
    const { socketMessageName, socketMessage, ...rest } = action;
    if (!socketMessageName) {
      return next(action);
    }

    socket.send(socketMessage.programInstructions);
  };
}

