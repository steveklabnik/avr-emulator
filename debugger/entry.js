var React = require('react');
var ReactDOM = require('react-dom');
import { createStore, combineReducers } from 'redux';
import { Provider } from 'react-redux';

import EmulatorApp from './containers/EmulatorApp';
import * as reducers from './reducers';

const reducer = combineReducers(reducers);
const store = createStore(reducer);

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
