import React, { PropTypes } from 'react';

export default class ProgramInstructions {
  render() {
    const { performInstructions } = this.props;

    return (
      <div className="program-input-container">
        <form onSubmit={performInstructions} > 
          <input id="program-type" />
          <button id="submit-instructions" onClick={performInstructions} >Evaluate</button>
        </form>
      </div>
    );
  }
}
// ProgramInstructions.propTypes = {
//   registers: React.PropTypes.arrayOf(React.PropTypes.string).isRequired
// };