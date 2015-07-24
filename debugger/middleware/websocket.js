import { socket } from '../initializers/websocket';

export default function websocketMiddleware({ dispatch, getState }) {
  return (next) => (action) => {
    const { socketMessageName, socketMessage, ...rest } = action;

  	// if (typeof action === 'function') {
  	// 	// debugger
  	// 	// socket.send(socketMessage.programInstructions);
  	// 	action(action, getState)
  	// 	console.log('asdfasd')
  	// }


    if (!socketMessageName) {
      return next(action);
    }

    socket.send(socketMessage.programInstructions);
  };
}

