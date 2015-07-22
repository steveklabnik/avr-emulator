var React = require('react');

import { createStore, combineReducers, applyMiddleware } from 'redux';
import { Provider } from 'react-redux';

import EmulatorApp from './containers/EmulatorApp';
import * as reducers from './reducers';
import websocketMiddleware from './middleware/websocket';
import { initializeSocketListener } from './initializers/websocket';

const reducer = combineReducers(reducers);
const store = applyMiddleware(websocketMiddleware)(createStore)(reducer);
initializeSocketListener(store);

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
