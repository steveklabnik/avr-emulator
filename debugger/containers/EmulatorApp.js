import React from 'react';
import { bindActionCreators } from 'redux';
import { Connector } from 'react-redux';
import RegisterState from '../components/RegisterState';
import DebuggerButtons from '../components/DebuggerButtons';
import * as DebuggerActions from '../actions/DebuggerActions';

export default class EmulatorApp {
  render() {
    return (
      <Connector>
        {({ emulator, dispatch }) =>
          <div>
            <DebuggerButtons {...bindActionCreators(DebuggerActions, dispatch)} />
            <RegisterState registers={emulator.registers} />
          </div>
        }
      </Connector>
    );
  }
}
