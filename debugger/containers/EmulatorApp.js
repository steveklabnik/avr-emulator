import React from 'react';
import { bindActionCreators } from 'redux';
import { Connector } from 'react-redux';
import RegisterState from '../components/RegisterState';
import DebuggerButtons from '../components/DebuggerButtons';
import AssemblyProgram from '../components/AssemblyProgram';
import * as DebuggerActions from '../actions/DebuggerActions';

export default class EmulatorApp {
  render() {
    return (
      <Connector>
        {({ emulator, dispatch }) =>
          <div>
            <DebuggerButtons emulator={emulator}
              {...bindActionCreators(DebuggerActions, dispatch)} />
            <RegisterState registers={emulator.data_memory.registers} />
            <AssemblyProgram instructions={emulator.instructions} programPointer={emulator.program_pointer} />
          </div>
        }
      </Connector>
    );
  }
}
