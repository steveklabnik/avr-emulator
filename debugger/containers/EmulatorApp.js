import React from 'react';
import { bindActionCreators } from 'redux';
import { Connector } from 'react-redux';
import RegisterState from '../components/RegisterState';
import IOStore from '../components/IOStore';
import DebuggerButtons from '../components/DebuggerButtons';
import AssemblyProgram from '../components/AssemblyProgram';
import * as DebuggerActions from '../actions/DebuggerActions';

export default class EmulatorApp {
  render() {
    return (
      <Connector>
        {({ emulator, programRunner, dispatch }) =>
          <div className="row">
            <div className="col-sm-4">
              <h2>Program</h2>
              <DebuggerButtons programRunner={programRunner}
                {...bindActionCreators(DebuggerActions, dispatch)} />
              <AssemblyProgram instructions={emulator.instructions} programPointer={emulator.program_pointer} />
            </div>
            <div className="col-sm-8">
              <IOStore data={emulator.data_memory.io} />
              <RegisterState registers={emulator.data_memory.registers} />
            </div>
          </div>
        }
      </Connector>
    );
  }
}
