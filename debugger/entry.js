var React = require('react');
require("./stylesheets/main.scss");

import { createStore, combineReducers, applyMiddleware } from 'redux';
import { Provider } from 'react-redux';
import thunk from 'redux-thunk'

import EmulatorApp from './containers/EmulatorApp';
import * as reducers from './reducers';
import websocketMiddleware from './middleware/websocket';
import { initializeSocketListener } from './initializers/websocket';

console.log(reducers);
const reducer = combineReducers(reducers);
const store = applyMiddleware(thunk, websocketMiddleware)(createStore)(reducer);
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
