import './TableCell.css';
import React, {Component} from 'react';
import {Rect, InsertCell, GetCellsRequest, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

class TableCell extends React.Component {

  constructor(props) {
      super(props);

      this.onKeyDown = props.onKeyDown
      this.onFocus = props.onFocus
      this.state = {selected: false};

      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleFocus = this.handleFocus.bind(this);
      this.handleFocusOut = this.handleFocusOut.bind(this);
      this.handleValueChanged = this.handleValueChanged.bind(this);
  }

  render() {
      let classNames = "TableCell"
       if (this.state.selected) {
           classNames += " SelectedTableCell"
       }
      return (
          <td onFocus={this.handleFocus}  onBlur={this.handleFocusOut} onKeyDown={this.handleKeyDown} style={{ "max-width": "72px", "min-width": "72px" }}>
            <div className={classNames} height="100%" width="92%" onInput={this.handleValueChanged} contentEditable='true'>
                { this.props.cell.displayValue }
            </div>
          </td>
      )
  }

  handleValueChanged(e) {
      this.props.onChanged(this.props.cell, e.target.innerText);
  }

  handleFocus(e) {
      this.setState({selected: true});
      e.target.innerText = this.props.cell.value;
      this.props.onChanged(this.props.cell, e.target.innerText);
  }

  handleFocusOut(e) {
      this.setState({selected: false});
      e.target.innerText = this.props.cell.displayValue;
  }

  handleKeyDown(e) {
    if (e.keyCode !== 13) {
          return;
    }
    e.preventDefault();

    let target = e.target;
    this.onKeyDown(this.props.cell.row, this.props.cell.col, target.textContent, target);
  }
}

export default TableCell;
