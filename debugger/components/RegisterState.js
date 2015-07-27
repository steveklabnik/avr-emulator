import React, { PropTypes } from 'react';
import { formatHex, formatBinary, padInteger } from './helpers'

export default class RegisterState {

  render() {
    const { registers } = this.props;

    return (
      <div className="row registers panel">
        <div className="col-sm-12">
          <h2>Registers</h2>
        </div>
        <div className="col-sm-4">
          <table>
              <tr>
                <th>R</th>
                <th>Hex</th>
                <th>Bin</th>
                <th>Val</th>
              </tr>
            {registers.slice(0, 16).map((register, i) =>
              <tr>
                <td><strong>R{padInteger(i, "0", 2, 10)}</strong></td>
                <td>{formatHex(register)}</td>
                <td>{formatBinary(register)}</td>
                <td>{padInteger(register, "0", 3, 10)}</td>
              </tr>
            )}
          </table>
        </div>
        <div className="col-sm-4">
          <table>
              <tr>
                <th>R</th>
                <th>Hex</th>
                <th>Bin</th>
                <th>Val</th>
              </tr>
            {registers.slice(15, -1).map((register, i) =>
              <tr>
                <td><strong>R{padInteger(i+16, "0", 2, 10)}</strong></td>
                <td>{formatHex(register)}</td>
                <td>{formatBinary(register)}</td>
                <td>{padInteger(register, "0", 3, 10)}</td>
              </tr>
            )}
          </table>
        </div>
      </div>
    );
  }
}
RegisterState.propTypes = {
  registers: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

