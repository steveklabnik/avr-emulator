import React from 'react';
import { bindActionCreators } from 'redux';
import { Connector } from 'react-redux';
import RegisterState from '../components/RegisterState';
import ProgramStack from '../components/ProgramStack';
import DebuggerButtons from '../components/DebuggerButtons';
import * as DebuggerActions from '../actions/DebuggerActions';

export default class EmulatorApp {
  render() {
    return (
      <Connector>
        {({ emulator, dispatch }) =>
          <div>
            <DebuggerButtons emulator={emulator}
              {...bindActionCreators(DebuggerActions, dispatch)} />
            <RegisterState registers={emulator.registers} />
            <ProgramStack stack={emulator.stack} />
          </div>
        }
      </Connector>
    );
  }
}
