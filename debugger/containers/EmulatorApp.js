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
                    Starfighters Summer 2015 announcement</a>,
                    we thought implementing our own Emulator would be a great problem to tackle
                    to dive a little deeper into with a few technologies
                    we wanted to get our hands on.
                    During our last biweekly hack day,
                    we dove in and few days later, we had a barebones working version.
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
