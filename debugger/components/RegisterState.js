import React, { PropTypes } from 'react';
import { formatHex, padInteger } from './helpers'

export default class RegisterState {

  render() {
    const { registers } = this.props;

    return (
      <div>
        <h2>Registers</h2>
        <table>
          {registers.map((register, i) =>
            <tr>
              <td>R{padInteger(i, "0", 2)}</td>
              <td>{formatHex(register)}</td>
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

