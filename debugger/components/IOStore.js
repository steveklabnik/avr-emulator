import React, { PropTypes } from 'react';
import { formatHex, formatBinary, padInteger } from './helpers'

export default class IOStore {

  render() {
    const { data } = this.props;

    return (
      <div>
        <h2>I/O Registers</h2>
        <table>
          <tr>
            <td>$3f</td>
            <td>SREG</td>
            <td>{data[63]}</td>
            <td>{formatBinary(data[63])}</td>
          </tr>
        </table>

        <table>
          {data.map((value, address) =>
            <tr>
              <td>${padInteger(address, "0", 2)}</td>
              <td>{formatBinary(value)}</td>
            </tr>
          )}
        </table>
      </div>
    );
  }
}
IOStore.propTypes = {
  data: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

