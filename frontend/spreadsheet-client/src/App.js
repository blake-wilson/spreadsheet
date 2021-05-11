import logo from './logo.svg';
import './App.css';
import TableCell from './TableCell';
import {Rect} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

import {GetCellsRequest, InsertCell, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import React, {Component} from 'react';

let hostname = "spreadsheet.yellowpapersun.net"
let apiClient = new SpreadsheetAPIClient('https://' + hostname + ':8080',
                               null, null);

class App extends React.Component {

  constructor(props) {
      super(props);

      this.handleKeyDown = this.handleKeyDown.bind(this);
      const items = [];
      for (let i = 0; i < this.props.numRows; i++) {
          const cells = [];
          for (let j = 0; j < this.props.numCols; j++) {
              let cell = <TableCell key={i*this.props.numCols + j} width="72px" height="24px" row={i} col={j} value="" displayValue = "" onKeyDown={this.handleKeyDown} />;
              items.push(cell);
          }
      }
      this.state = {table: items, numRows: this.props.numRows, numCols: this.props.numCols};

      // get the initial cells values
      var request = new GetCellsRequest();
      var rect = new Rect();
      rect.setStartRow(0);
      rect.setStopRow(100);
      rect.setStartCol(0);
      rect.setStopCol(20);
      console.log("rect: ",rect);

      request.setRect(rect);
      apiClient.getCells(request, {}, (err, response) => {
      if (err) {
          console.log(`Unexpected error for insertCells: code = ${err.code}` +
                      `, message = "${err.message}"`);
        } else {
            this.insertIntoTable(response.getCellsList());
        }
      });
  }

  insertIntoTable(cells) {
    for (const c of cells) {
      var row = c.getRow();
      var col = c.getCol();
      let table = [...this.state.table];
      let idx = row * this.props.numCols + col
      let tableCell = {...table[idx], value: c.getValue(), displayValue: c.getDisplayValue()};
      tableCell.props = {...tableCell.props, value: c.getValue(), displayValue: c.getDisplayValue()};
      console.log(tableCell);
      table[idx] = tableCell;
      console.log(idx);
      this.setState({table: table});
    }
  }

  handleKeyDown(row, col, textContent, cellElement) {
    var c1 = new InsertCell();
    c1.setRow(row);
    c1.setCol(col);
    c1.setValue(textContent);
    var request = new InsertCellsRequest();
    request.setCellsList([c1]);
    apiClient.insertCells(request, {}, (err, response) => {
    if (err) {
        console.log(`Unexpected error for insertCells: code = ${err.code}` +
                    `, message = "${err.message}"`);
      } else {
          console.log("inserted " + response.getCellsList() + " cells");
          this.insertIntoTable(response.getCellsList());
      }
    });
  }


  render() {
    let items = [];
    let header_items = [<td> </td>];
    for (let i = 0; i < this.props.numCols; i++) {
        header_items.push(<td>{ String.fromCharCode('A'.charCodeAt() + i) }</td>);
        console.log("added header item");
    }

    for (let i = 0; i < this.props.numRows; i++) {
        items.push(
            <tr>
            <td>{ (i + 1).toString()}</td>
            { this.state.table.slice(
            i * this.props.numCols,
            i * this.props.numCols + this.props.numCols)
            }
        </tr>);
   }
    return (
      <div className="App">
        <header className="App-header">
          <table border="1px solid white"> 
            <tr>
                { header_items }
            </tr>
            {items}
          </table>
        </header>
      </div>
    );
  }
}

export default App;
