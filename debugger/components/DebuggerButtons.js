import React, { PropTypes } from 'react';

export default class DebuggerButtons {

  render() {
    const { performStep } = this.props;
    return (
      <div>
        <h2>Debugger Actions</h2>
        <p>
          <button onClick={performStep}>Step</button>
        </p>
      </div>
    );
  }
}
DebuggerButtons.propTypes = {
  performStep: PropTypes.func.isRequired,
  emulator: PropTypes.object.isRequired
};
