var React = require('react');
var ReactDOM = require('react-dom');
import { createStore, combineReducers, applyMiddleware } from 'redux';
import { Provider } from 'react-redux';

import EmulatorApp from './containers/EmulatorApp';
import * as reducers from './reducers';
import websocketMiddleware from './middleware/websocket';

import { socket } from './socket'

const reducer = combineReducers(reducers);
const store = applyMiddleware(websocketMiddleware)(createStore)(reducer);

socket.onmessage = function (event) {
  console.log('received from websocket', event);
  store.dispatch({
    type: 'performNext',
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

ReactDOM.render(<EmulatorProvider />, document.getElementById('emulator'));
