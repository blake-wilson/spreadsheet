import './App.css';
import TableCell from './TableCell';
import FormulaBar from './FormulaBar';
import {Rect} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

import {GetCellsRequest, InsertCell, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import React, {Component} from 'react';

//let hostname = "https://spreadsheet.yellowpapersun.net"
let hostname = 'http://localhost:8080';
let apiClient = new SpreadsheetAPIClient(hostname,
                               null, null);

class App extends React.Component {

  constructor(props) {
      super(props);

      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleTableCellFocus = this.handleTableCellFocus.bind(this);
      this.handleFormulaBarChanged = this.handleFormulaBarChanged.bind(this);
      this.handleCellInsert = this.handleCellInsert.bind(this);

      const items = [];
      for (let i = 0; i < this.props.numRows; i++) {
          const cells = [];
          for (let j = 0; j < this.props.numCols; j++) {
              let cell = <TableCell key={i*this.props.numCols + j}
                          width="72px" height="24px" row={i} col={j} value=""
                          displayValue = "" onKeyDown={this.handleKeyDown}
                          onFocus={this.handleTableCellFocus} />;
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
      table[idx] = tableCell;
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

  handleCellInsert(cell) {
    var c1 = new InsertCell();
    c1.setRow(cell.row);
    c1.setCol(cell.col);
    c1.setValue(cell.value);
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

  handleTableCellFocus(row, col, textContent) {
     this.setState({selectedCell: {
         row: row,
         col: col,
         value: textContent
     }});
  }

  handleFormulaBarChanged(value) {
      let selectedCell = {...this.state.selectedCell, value: value};
      this.setState({selectedCell: selectedCell});
  }

  render() {
    let items = [];
    let header_items = [<td> </td>];
    for (let i = 0; i < this.props.numCols; i++) {
        header_items.push(<td>{ String.fromCharCode('A'.charCodeAt() + i) }</td>);
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
        <div>
          <FormulaBar cell={ this.state.selectedCell } onChanged={this.handleFormulaBarChanged} 
                            onSubmit={this.handleCellInsert} />
            <table border="1px solid white"> 
              <tr>
                  { header_items }
              </tr>
              {items}
            </table>
          </div>
        </header>
      </div>
    );
  }
}

export default App;
