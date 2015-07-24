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
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[0] === "1" ? "blue" : "grey")}}>I</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[1] === "1" ? "blue" : "grey")}}>T</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[2] === "1" ? "blue" : "grey")}}>H</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[3] === "1" ? "blue" : "grey")}}>S</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[4] === "1" ? "blue" : "grey")}}>V</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[5] === "1" ? "blue" : "grey")}}>N</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[6] === "1" ? "blue" : "grey")}}>Z</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x3f])[7] === "1" ? "blue" : "grey")}}>C</td>
          </tr>
          <tr>
            <td>$18</td>
            <td>PORTB</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[0] === "1" ? "blue" : "grey")}}>PORTB7</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[1] === "1" ? "blue" : "grey")}}>PORTB6</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[2] === "1" ? "blue" : "grey")}}>PORTB5</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[3] === "1" ? "blue" : "grey")}}>PORTB4</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[4] === "1" ? "blue" : "grey")}}>PORTB3</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[5] === "1" ? "blue" : "grey")}}>PORTB2</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[6] === "1" ? "blue" : "grey")}}>PORTB1</td>
            <td colSpan="2" style={{color: (formatBinary(data[0x18])[7] === "1" ? "blue" : "grey")}}>PORTB0</td>
          </tr>
          <tr>
            <td>$18</td>
            <td>PORTB</td>
            <td style={{color: "grey"}}>PORTB7</td><td>{formatBinary(data[0x18])[0]}</td>
            <td style={{color: "grey"}}>PORTB6</td><td>{formatBinary(data[0x18])[1]}</td>
            <td style={{color: "grey"}}>PORTB5</td><td>{formatBinary(data[0x18])[2]}</td>
            <td style={{color: "grey"}}>PORTB4</td><td>{formatBinary(data[0x18])[3]}</td>
            <td style={{color: "grey"}}>PORTB3</td><td>{formatBinary(data[0x18])[4]}</td>
            <td style={{color: "grey"}}>PORTB2</td><td>{formatBinary(data[0x18])[5]}</td>
            <td style={{color: "grey"}}>PORTB1</td><td>{formatBinary(data[0x18])[6]}</td>
            <td style={{color: "grey"}}>PORTB0</td><td>{formatBinary(data[0x18])[7]}</td>
          </tr>
        </table>
      </div>
    );
  }
}
IOStore.propTypes = {
  data: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};

