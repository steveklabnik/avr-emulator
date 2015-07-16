import React, { PropTypes } from 'react';

export default class RegisterState {

  render() {
    const { registers } = this.props;
    return (
      <ul>
        {registers.map(register =>
          <li>{register}</li>
        )}
      </ul>
    );
  }
}
RegisterState.propTypes = {
  registers: React.PropTypes.arrayOf(React.PropTypes.string).isRequired
};

