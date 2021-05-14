import {Rect} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

import {GetCellsRequest, InsertCell, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import React, {Component} from 'react';

class App extends React.Component {

  constructor(props) {
      super(props);

      this.handleChange = this.handleChange.bind(this);
      this.handleKeyDown = this.handleKeyDown.bind(this);

      this.onSubmit = props.onSubmit
  }

  handleKeyDown(e) {
    if (e.keyCode !== 13) {
          return;
    }
    e.preventDefault();

    this.onSubmit(this.props.cell);
  }
  
  handleChange(e) {
      this.props.onChanged(e.target.value);
  }

  render() {
    let value = "";
    if (this.props.cell !== undefined) {
        value = this.props.cell.value;
    }
    return (
      <div className="FormulaBar">
        <input value={value} onKeyDown={this.handleKeyDown} onChange={this.handleChange} style={{ width: "100%" }} />
      </div>
    );
  }
}

export default App;
