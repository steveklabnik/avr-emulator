import React, { PropTypes } from 'react';

export default class DebuggerButtons {

  render() {
    const { performStep, executeProgram } = this.props;

    return (
      <div>
        <h2>Debugger Actions</h2>
        <div className='controls'>
          <button onClick={performStep}>Step</button>
          <button onClick={executeProgram}>Execute Program</button>
        </div>
      </div>
    );
  }
}
DebuggerButtons.propTypes = {
  performStep: PropTypes.func.isRequired,
  executeProgram: PropTypes.func.isRequired,
  emulator: PropTypes.object.isRequired
};
