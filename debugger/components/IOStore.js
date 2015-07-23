import React, { PropTypes } from 'react';
import { formatHex, padInteger } from './helpers'

export default class IOStore {

  render() {
    const { data } = this.props;

    return (
      <div>
        <h2>I/O Registers</h2>
        <table>
          {data.map((value, address) =>
            <tr>
              <td>R{padInteger(address, "0", 2)}</td>
              <td>{formatHex(value)}</td>
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

