import React, { PropTypes } from 'react';

export default class DebuggerButtons {

  render() {
    const { requestProgram } = this.props;
    return (
      <p>
        <button onClick={requestProgram}>Load Program</button>
      </p>
    );
  }
}
DebuggerButtons.propTypes = {
  requestProgram: PropTypes.func.isRequired,
  emulator: PropTypes.object.isRequired
};
