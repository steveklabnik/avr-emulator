import React from 'react';
import { Connector } from 'react-redux';
import RegisterState from '../components/RegisterState';

export default class EmulatorApp {
  render() {
    return (
      <Connector>
        {({ emulator }) =>
          /* Yes this is child as a function. */
          <RegisterState registers={emulator.registers} />
        }
      </Connector>
    );
  }
}
