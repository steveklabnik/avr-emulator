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
            <th>Address</th>
            <th>Name</th>
            <th>Bit 7</th>
            <th>Bit 6</th>
            <th>Bit 5</th>
            <th>Bit 4</th>
            <th>Bit 3</th>
            <th>Bit 2</th>
            <th>Bit 1</th>
            <th>Bit 0</th>
          </tr>
          <tr>
            <td>$3f</td>
            <td>SREG</td>
            <td>I:{formatBinary(data[63])[0]}</td>
            <td>T:{formatBinary(data[63])[1]}</td>
            <td>H:{formatBinary(data[63])[2]}</td>
            <td>S:{formatBinary(data[63])[3]}</td>
            <td>V:{formatBinary(data[63])[4]}</td>
            <td>N:{formatBinary(data[63])[5]}</td>
            <td>Z:{formatBinary(data[63])[6]}</td>
            <td>C:{formatBinary(data[63])[7]}</td>
          </tr>
        </table>
      </div>
    );
  }
}
IOStore.propTypes = {
  data: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

