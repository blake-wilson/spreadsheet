import './TableCell.css';
import React, {Component} from 'react';
import {Rect, InsertCell, GetCellsRequest, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

class TableCell extends React.Component {

  constructor(props) {
      super(props);

      this.onKeyDown = props.onKeyDown
      this.state = {
          selected: false,
      };

      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleFocus = this.handleFocus.bind(this);
      this.handleBlur = this.handleBlur.bind(this);
      this.handleValueChanged = this.handleValueChanged.bind(this);
  }

  handleBlur(e) {
      this.setState({
          selected: false
      });
  }

  render() {
      let classNames = "TableCell"
      if (this.state !== undefined && this.state.selected) {
          classNames += " SelectedTableCell"
      }
      let displayValue = this.props.cell.displayValue;
      return (
          <td className={classNames} onBlur={this.handleBlur} onFocus={this.handleFocus} onKeyDown={this.handleKeyDown}>
           <input class="TableInput TableTd" ref={this.props.tableRef} height="100%" onChange={this.handleValueChanged} contentEditable='true'
                value={ this.state.selected ? this.props.cell.value : this.props.cell.displayValue } />
          </td>
      )
  }

  handleValueChanged(e) {
      this.props.onChanged(this.props.cell, e.target.value);
  }

  handleFocus(e) {
      this.setState({selected: true});
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
        value: e.target.value,
    }
    this.onKeyDown(cell);
  }
}

export default TableCell;
