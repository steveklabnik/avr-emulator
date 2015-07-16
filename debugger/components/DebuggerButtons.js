import React, { PropTypes } from 'react';

export default class DebuggerButtons {
  programIsLoaded() {
    const { emulator } = this.props;
    return emulator.stack.length > 0;
  }

  render() {
    const { loadProgram, performNextOperation } = this.props;
    return (
      <p>
        { this.programIsLoaded() ? <button onClick={performNextOperation}>Step</button> : null }
        <button onClick={loadProgram}>Load Program</button>
      </p>
    );
  }
}
DebuggerButtons.propTypes = {
  performNextOperation: PropTypes.func.isRequired,
  loadProgram: PropTypes.func.isRequired,
  emulator: PropTypes.object.isRequired
};

