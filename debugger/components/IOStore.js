import React, { PropTypes } from 'react';
import { formatHex, formatBinary, padInteger } from './helpers'
import IORegister from '../components/IORegister';

export default class IOStore {

  render() {
    const { data } = this.props;

    return (
      <div className="io-store">
        <h2>I/O Registers</h2>
        <table>
          <tr>
            <th>Address</th>
            <th>Name</th>
            <th>Hex</th>
            <th colSpan="2">Bit 7</th>
            <th colSpan="2">Bit 6</th>
            <th colSpan="2">Bit 5</th>
            <th colSpan="2">Bit 4</th>
            <th colSpan="2">Bit 3</th>
            <th colSpan="2">Bit 2</th>
            <th colSpan="2">Bit 1</th>
            <th colSpan="2">Bit 0</th>
          </tr>
          <IORegister address="$3f"
            data={data[0x3f]}
            name="SREG"
            bitNames={['I', 'T', 'H', 'S', 'V', 'N', 'Z', 'C']} />
          <IORegister address="$18"
            data={data[0x18]}
            name="PORTB"
            bitNames={['PORTB7','PORTB6','PORTB5','PORTB4','PORTB3','PORTB2','PORTB1','PORTB0']} />
        </table>
      </div>
    );
  }
}
IOStore.propTypes = {
  data: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

