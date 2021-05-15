import formulaImage from './formula.png';
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
      <div className="FormulaBar" style = {{ "margin-bottom": "10px", "margin-top": "10px" }}>
        <img src={formulaImage} height="28px" /><input value={value} onKeyDown={this.handleKeyDown} onChange={this.handleChange}
               style={{ width: "90%", "font-size": "22px" }} />
      </div>
    );
  }
}

export default App;
