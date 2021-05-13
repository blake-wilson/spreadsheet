import React, {Component} from 'react';
import {Rect, InsertCell, GetCellsRequest, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

class TableCell extends React.Component {

  constructor(props) {
      super(props);

      this.onKeyDown = props.onKeyDown
      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleFocus = this.handleFocus.bind(this);
      this.handleFocusOut = this.handleFocusOut.bind(this);
  }

  render() {
      return (
          <td onFocus={this.handleFocus} onBlur={this.handleFocusOut} onKeyDown={this.handleKeyDown} contentEditable='true' height="20px" width="72px">
            { this.props.displayValue }
          </td>
      )
  }

  handleFocus(e) {
      e.target.innerText = this.props.value;
  }

  handleFocusOut(e) {
      e.target.innerText = this.props.displayValue;
  }

  handleKeyDown(e) {
    if (e.keyCode !== 13) {
          return;
    }
    e.preventDefault();

    var c1 = new InsertCell();
    let target = e.target;
    c1.setRow(this.props.row);
    c1.setCol(this.props.col);
    c1.setValue(target.textContent);
    this.onKeyDown(this.props.row, this.props.col, target.textContent, target);
  }
}

export default TableCell;
