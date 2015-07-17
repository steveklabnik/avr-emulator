import { socket } from '../socket';

export default function websocketMiddleware({ dispatch, getState }) {
  return (next) => (action) => {
    const { socketMessageName, socketMessage, ...rest } = action;
    if (!socketMessageName) {
      return next(action);
    }
    socket.send('requesting: ' + socketMessageName);
  };
}

