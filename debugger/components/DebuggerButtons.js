import React, { PropTypes } from 'react';

export default class DebuggerButtons {

  handleStop = () => {
    const { stopExecution, emulator } = this.props;
    console.log(emulator);
    stopExecution(emulator.executionId);
  }

  render() {
    const { performStep, executeProgram } = this.props;

    return (
      <div>
        <h2>Debugger Actions</h2>
        <div className='controls'>
          <button onClick={performStep}>Step</button>
          <button onClick={executeProgram}>Run</button>
          <button onClick={this.handleStop}>Stop</button>
        </div>
        <p>{emulator.executionId}</p>
      </div>
    );
  }
}
DebuggerButtons.propTypes = {
  performStep: PropTypes.func.isRequired,
  executeProgram: PropTypes.func.isRequired,
  stopExecution: PropTypes.func.isRequired,
  emulator: PropTypes.object.isRequired
};
