import React, { PropTypes } from 'react';

export default class RegisterState {

  render() {
    const { registers } = this.props;
    
    return (
      <div>
        <h2>Registers</h2>
        <ul>
          {registers.map(register =>
            <li>{register}</li>
          )}
        </ul>
      </div>
    );
  }
}
RegisterState.propTypes = {
  registers: React.PropTypes.arrayOf(React.PropTypes.string).isRequired
};

