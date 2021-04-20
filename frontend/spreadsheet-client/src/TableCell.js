import React, {Component} from 'react';
import {InsertCell, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

let apiClient = new SpreadsheetAPIClient('http://' + window.location.hostname + ':8080',
                               null, null);

class TableCell extends React.Component {

  constructor(props) {
      super(props);
      this.state = {value: "", displayValue: ""};

      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleFocus = this.handleFocus.bind(this);
      this.handleFocusOut = this.handleFocusOut.bind(this);
  }

  render() {
      return (
          <td onFocus={this.handleFocus} onBlur={this.handleFocusOut} onKeyDown={this.handleKeyDown} contentEditable='true' height="20px" width="72px">
            { this.state.value }
          </td>
      )
  }

  handleFocus(e) {
      e.target.innerText = this.state.value;
  }

  handleFocusOut(e) {
      e.target.innerText = this.state.displayValue;
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
    var request = new InsertCellsRequest();
    request.setCellsList([c1]);
    apiClient.insertCells(request, {}, (err, response) => {
    if (err) {
        console.log(`Unexpected error for insertCells: code = ${err.code}` +
                    `, message = "${err.message}"`);
      } else {
          console.log("inserted " + response.getCellsList() + " cells");
          for (const c of response.getCellsList()) {
              console.log(c.getValue());
              target.innerText = c.getDisplayValue()
              this.state.value = c.getValue();
              this.state.displayValue = c.getDisplayValue();
          }
      }
    });
  }
}

export default TableCell;
