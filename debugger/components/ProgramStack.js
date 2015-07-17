import React, { PropTypes } from 'react';

export default class ProgramStack {

  render() {
    const { stack } = this.props;
    return (
      <div>
        <h2>Stack</h2>
        <ul>
          {stack.map(memoryAddress =>
            <li key={memoryAddress} >{memoryAddress}</li>
          )}
        </ul>
      </div>
    );
  }
}

ProgramStack.propTypes = {
  stack: React.PropTypes.arrayOf(React.PropTypes.number).isRequired
};
