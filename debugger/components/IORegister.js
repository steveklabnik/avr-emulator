import React from 'react';
import { formatHex, formatBinary } from './helpers'
import classnames from 'classnames';

export default class IORegister {

  render() {
    const { address, name, data, bitNames } = this.props;

    var assignClasses = function assignClasses(i) {
      return classnames('io-num', {
        'current-store': i === "1"
      })
    }

    var formattedBinary = formatBinary(data);

    return (
      <tr>
        <td>{address}</td>
        <td>{name}</td>
        <td>{formatHex(data)}</td>
        {formattedBinary.split("").map((bitValue, i) =>
          <td colSpan="2" className={assignClasses(bitValue)}>{bitNames[i]}</td>
        )}
      </tr>
    );
  }
}
