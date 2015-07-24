import React, { PropTypes } from 'react';
import { formatHex, formatBinary, padInteger } from './helpers'
import classnames from 'classnames';

export default class IOStore {

  render() {
    const { data } = this.props;

    var assignClasses = function assignClasses(i) {
      return classnames('io-num', {
        'current-store': i === "1" 
      })
    }

    return (
      <div>
        <h2>I/O Registers</h2>
        <table>
          <tr>
            <th>Address</th>
            <th>Name</th>
            <th colSpan="2">Bit 7</th>
            <th colSpan="2">Bit 6</th>
            <th colSpan="2">Bit 5</th>
            <th colSpan="2">Bit 4</th>
            <th colSpan="2">Bit 3</th>
            <th colSpan="2">Bit 2</th>
            <th colSpan="2">Bit 1</th>
            <th colSpan="2">Bit 0</th>
          </tr>
          <tr>
            <td>$3f</td>
            <td>SREG</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[0])}>I</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[1])}>T</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[2])}>H</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[3])}>S</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[4])}>V</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[5])}>N</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[6])}>Z</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[7])}>C</td>
          </tr>
          <tr>
            <td>$18</td>
            <td>PORTB</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[0])}>PORTB7</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[1])}>PORTB6</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[2])}>PORTB5</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[3])}>PORTB4</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[4])}>PORTB3</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[5])}>PORTB2</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[6])}>PORTB1</td>
            <td colSpan="2" className={assignClasses(formatBinary(data[0x3f])[7])}>PORTB0</td>
          </tr>
          <tr>
            <td>$18</td>
            <td>PORTB</td>
            <td className={assignClasses()}>PORTB7</td><td>{formatBinary(data[0x18])[0]}</td>
            <td className={assignClasses()}>PORTB6</td><td>{formatBinary(data[0x18])[1]}</td>
            <td className={assignClasses()}>PORTB5</td><td>{formatBinary(data[0x18])[2]}</td>
            <td className={assignClasses()}>PORTB4</td><td>{formatBinary(data[0x18])[3]}</td>
            <td className={assignClasses()}>PORTB3</td><td>{formatBinary(data[0x18])[4]}</td>
            <td className={assignClasses()}>PORTB2</td><td>{formatBinary(data[0x18])[5]}</td>
            <td className={assignClasses()}>PORTB1</td><td>{formatBinary(data[0x18])[6]}</td>
            <td className={assignClasses()}>PORTB0</td><td>{formatBinary(data[0x18])[7]}</td>
          </tr>
        </table>
      </div>
    );
  }
}
IOStore.propTypes = {
  data: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

