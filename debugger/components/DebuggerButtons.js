import React, { PropTypes } from 'react';

export default class Counter {

  render() {
    const { performNextOperation } = this.props;
    return (
      <p>
        <button onClick={performNextOperation}>Step</button>
      </p>
    );
  }
}
Counter.propTypes = {
  performNextOperation: PropTypes.func.isRequired
};

