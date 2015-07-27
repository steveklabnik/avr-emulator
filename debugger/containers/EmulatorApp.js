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
            <div className="col-sm-12">
              <h1 className='main-title'>Atmel AVR 8-bit Emulator in React and Rust</h1>
            </div>
            <div className="col-sm-4">

              <div className='program panel'>
                <h2>Program</h2>
                <DebuggerButtons programRunner={programRunner}
                  {...bindActionCreators(DebuggerActions, dispatch)} />
                <AssemblyProgram instructions={emulator.instructions} programPointer={emulator.program_pointer} />
              </div>

              <div className='panel'>
                <h2>About</h2>
                <p>
                  Inspired by&nbsp;
                  <a href="http://sockpuppet.org/blog/2015/07/13/starfighter/">
                    Starfighter's Summer 2015 announcement
                  </a>,
                  we thought that implementing our own&nbsp;
                  <a href="http://www.atmel.com/images/atmel-0856-avr-instruction-set-manual.pdf">
                    Atmel AVR 8-bit
                  </a>
                  &nbsp;emulator would be a pretty fun problem to tackle.
                  Built with React.js, Redux.js, Rust and Docker
                  during our last&nbsp;
                  <a href="https://twitter.com/chrisconley/status/618830194971774976">
                    biweekly hack day
                  </a>.
                </p>
              </div>
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
