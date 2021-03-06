import './TableCell.css';
import React, {Component} from 'react';
import {Rect, InsertCell, GetCellsRequest, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

class TableCell extends React.Component {

  constructor(props) {
      super(props);

      this.onKeyDown = props.onKeyDown

      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleFocus = this.handleFocus.bind(this);
      this.handleValueChanged = this.handleValueChanged.bind(this);
  }

  render() {
      let classNames = "TableCell"
      if (this.props.selected) {
          classNames += " SelectedTableCell"
      }
      let displayValue = this.props.cell.displayValue;
      let enteredValue = this.props.enteredValue !== undefined ? this.props.enteredValue : this.props.cell.value;
      return (
          <td className={classNames} onFocus={this.handleFocus} onKeyDown={this.handleKeyDown}>
           <input class="TableInput TableTd" ref={this.props.tableRef} height="100%" onChange={this.handleValueChanged} contentEditable='true'
                value={ this.props.selected ? enteredValue : this.props.cell.displayValue } />
          </td>
      )
  }

  handleValueChanged(e) {
      this.props.onChanged(this.props.cell, e.target.value);
  }

  handleFocus(e) {
      this.props.onFocus(this.props.cell.row, this.props.cell.col, this.props.cell.value);
  }

  handleKeyDown(e) {
    if (e.keyCode !== 13) {
          return;
    }
    e.preventDefault();

    let target = e.target;
    this.onKeyDown(this.props.cell, this.props.enteredValue);
  }
}

export default TableCell;
