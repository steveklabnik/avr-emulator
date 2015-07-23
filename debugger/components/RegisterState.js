import React, { PropTypes } from 'react';
import { formatHex, formatBinary, padInteger } from './helpers'

export default class RegisterState {

  render() {
    const { registers } = this.props;

    return (
      <div>
        <h2>Registers</h2>
        <table>
            <tr>
              <th>R</th>
              <th>Hex</th>
              <th>Bin</th>
              <th>Val</th>
            </tr>
          {registers.map((register, i) =>
            <tr>
              <td>R{padInteger(i, "0", 2, 10)}</td>
              <td>{formatHex(register)}</td>
              <td>{formatBinary(register)}</td>
              <td>{padInteger(register, "0", 3, 10)}</td>
            </tr>
          )}
        </table>
      </div>
    );
  }
}
RegisterState.propTypes = {
  registers: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

