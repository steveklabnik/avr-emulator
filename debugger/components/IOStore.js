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
            <td>I:{formatBinary(data[0x3f])[0]}</td>
            <td>T:{formatBinary(data[0x3f])[1]}</td>
            <td>H:{formatBinary(data[0x3f])[2]}</td>
            <td>S:{formatBinary(data[0x3f])[3]}</td>
            <td>V:{formatBinary(data[0x3f])[4]}</td>
            <td>N:{formatBinary(data[0x3f])[5]}</td>
            <td>Z:{formatBinary(data[0x3f])[6]}</td>
            <td>C:{formatBinary(data[0x3f])[7]}</td>
          </tr>
          <tr>
            <td>$18</td>
            <td>PORTB</td>
            <td>PORTB7:{formatBinary(data[0x18])[0]}</td>
            <td>PORTB6:{formatBinary(data[0x18])[1]}</td>
            <td>PORTB5:{formatBinary(data[0x18])[2]}</td>
            <td>PORTB4:{formatBinary(data[0x18])[3]}</td>
            <td>PORTB3:{formatBinary(data[0x18])[4]}</td>
            <td>PORTB2:{formatBinary(data[0x18])[5]}</td>
            <td>PORTB1:{formatBinary(data[0x18])[6]}</td>
            <td>PORTB0:{formatBinary(data[0x18])[7]}</td>
          </tr>
        </table>
      </div>
    );
  }
}
IOStore.propTypes = {
  data: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

