import React, { PropTypes } from 'react';

export default class DebuggerButtons {
  // TODO: All/part of this should be on the reducer/store
  programIsLoaded() {
    const { emulator } = this.props;
    return emulator.stack.length > 0;
  }

  render() {
    const { requestProgram, requestNextOperation } = this.props;
    return (
      <p>
        { this.programIsLoaded() && <button onClick={requestNextOperation}>Step</button> }
        <button onClick={requestProgram}>Load Program</button>
      </p>
    );
  }
}
DebuggerButtons.propTypes = {
  requestNextOperation: PropTypes.func.isRequired,
  requestProgram: PropTypes.func.isRequired,
  emulator: PropTypes.object.isRequired
};

