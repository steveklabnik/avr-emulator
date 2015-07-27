import React, { PropTypes } from 'react';

export default class DebuggerButtons {

  handleStop = () => {
    const { stopExecution, programRunner } = this.props;
    stopExecution(programRunner.executionId);
  }

  render() {
    const { performStep, executeProgram, programRunner } = this.props;

    return (
      <div className="debugger-controls">
        <div className='controls'>
          <button disabled={programRunner.executionId} onClick={performStep}>Step</button>
          <button disabled={programRunner.executionId} onClick={executeProgram}>Run</button>
          <button disabled={!programRunner.executionId} onClick={this.handleStop}>Stop</button>
        </div>
      </div>
    );
  }
}
DebuggerButtons.propTypes = {
  performStep: PropTypes.func.isRequired,
  executeProgram: PropTypes.func.isRequired,
  stopExecution: PropTypes.func.isRequired,
  programRunner: PropTypes.object.isRequired
};
