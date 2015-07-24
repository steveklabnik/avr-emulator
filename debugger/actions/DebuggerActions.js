import { socket } from '../initializers/websocket';

export const WEBSOCKET_REQUEST = 'websocketRequest';
export const WEBSOCKET_UPDATE = 'websocketUpdate';

export const PERFORM_STEP = 'performStep';
export const EXECUTE_PROGRAM = 'executeProgram';

export function performStep() {
  return {
    type: WEBSOCKET_REQUEST,
    socketMessageName: PERFORM_STEP,
    socketMessage: {}
  };
}

export function executeProgram() {
	console.log('wtf')

  return (dispatch, getState) => {
    const { timerId } = getState();
    if (timerId === null) {
      const timerId = setInterval(() => {
        dispatch({
		    	type: WEBSOCKET_REQUEST,
		    	socketMessageName: PERFORM_STEP,
		    	socketMessage: {}
		  	}) // a store supposed to save `timerId`
        
        console.log('tick')
      }, 1000)
      dispatch({
		    type: WEBSOCKET_REQUEST,
		    socketMessageName: PERFORM_STEP,
		    socketMessage: {}
		  }) // a store supposed to save `timerId`
    }
  }
}