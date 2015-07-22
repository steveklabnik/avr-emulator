var React = require('react');

import { createStore, combineReducers, applyMiddleware } from 'redux';
import { Provider } from 'react-redux';

import EmulatorApp from './containers/EmulatorApp';
import * as reducers from './reducers';
import websocketMiddleware from './middleware/websocket';
import { WEBSOCKET_UPDATE } from './actions/DebuggerActions';

import { socket } from './socket'

const reducer = combineReducers(reducers);
const store = applyMiddleware(websocketMiddleware)(createStore)(reducer);

socket.onmessage = function (event) {
  console.log('received from websocket', event);
  store.dispatch({
    type: WEBSOCKET_UPDATE,
    event: event
  });
};

var EmulatorProvider = React.createClass({
  render: function() {
    return (
      <Provider store={store}>
        {() => <EmulatorApp />}
      </Provider>
    );
  }
});

React.render(<EmulatorProvider />, document.getElementById('emulator'));
