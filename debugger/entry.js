var React = require('react');
var ReactDOM = require('react-dom');
import { createStore, combineReducers } from 'redux';
import { Provider, Connector } from 'react-redux';

import * as reducers from './reducers';
const reducer = combineReducers(reducers);
const store = createStore(reducer);

var EmulatorApp = React.createClass({
  render: function() {
    return (
      <Provider store={store}>
        {() => (
          <Connector>
            {() => <div>hi</div>}
          </Connector>
        )}
      </Provider>
    );
  }
});

ReactDOM.render(<EmulatorApp />, document.getElementById('emulator'));
