import React, { PropTypes } from 'react';

export default class RegisterState {

  render() {
    const { instructions, programPointer } = this.props;

    return (
      <div>
        <h2>Program</h2>
        <code>
          {instructions.map((instruction, i) =>
            <p style={{"background-color": (i === programPointer ? "#ddd" : "white")}}>{instruction}</p>
          )}
        </code>
      </div>
    );
  }
}
RegisterState.propTypes = {
  instructions: React.PropTypes.arrayOf(React.PropTypes.string).isRequired,
  programPointer: React.PropTypes.number.isRequired,
};
