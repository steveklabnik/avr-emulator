import React, { PropTypes } from 'react';

export default class ProgramInstructions extends React.Component {

  handleEvaluateRequest = (e) => {
    const { performInstructions } = this.props;
    performInstructions('deprecated');
  }

  render() {
    return (
      <div className="program-input-container">
        <button id="submit-instructions" onClick={this.handleEvaluateRequest}>Evaluate</button>
      </div>
    );
  }

}
 ProgramInstructions.propTypes = {
   performInstructions: React.PropTypes.func.isRequired
 };
