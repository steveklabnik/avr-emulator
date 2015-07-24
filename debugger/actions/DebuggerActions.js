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
  // return dispatch => {
  //   setTimeout(() => {
  //     dispatch(performStep());
  //   }, 1000);
  // };

   //  return (dispatch, getState) => {
	  //   // const { timerId } = getState();
	  //   timerId = undefined	
	  //     const timerId = setInterval(() => {
	  //       dispatch(performStep())
	  //     }, 1000)
	  //     dispatch({type: "Nothing", payload: timerId}) // a store supposed to save `timerId`
	  //   }
	  // }
}