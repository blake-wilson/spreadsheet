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
      return (
          <td onFocus={this.handleFocus} onKeyDown={this.handleKeyDown}>
            <div ref={this.props.tableRef} className={classNames} height="100%" width="92%" onInput={this.handleValueChanged} contentEditable='true'>
                { this.props.selected ? this.props.cell.value : this.props.cell.displayValue }
            </div>
          </td>
      )
  }

  handleValueChanged(e) {
      this.props.onChanged(this.props.cell, e.target.innerText);
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
    let cell = {
        row: this.props.cell.row,
        col: this.props.cell.col,
        value: e.target.innerText,
    }
    console.log("calling handler for cell: ", cell);
    this.onKeyDown(cell);
    console.log("curr row:", this.props.cell.row, "new row", (this.props.cell.row + 1) % 20);
    this.props.onFocus((this.props.cell.row + 1) % 20, this.props.cell.col, cell.value);
  }
}

export default TableCell;
