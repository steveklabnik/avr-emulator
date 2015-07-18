import React, { PropTypes } from 'react';

export default class ProgramInstructions extends React.Component {

  handleInstructionChange = (e) => {
    this.setState({instruction: e.target.value});
  }

  handleEvaluateRequest = (e) => {
    const { performInstructions } = this.props;
    performInstructions(this.state.instruction);
  }

  render() {
    return (
      <div className="program-input-container">
        <input ref="program-type" onChange={this.handleInstructionChange} />
        <button id="submit-instructions" onClick={this.handleEvaluateRequest}>Evaluate</button>
      </div>
    );
  }

}
 ProgramInstructions.propTypes = {
   performInstructions: React.PropTypes.func.isRequired
 };
