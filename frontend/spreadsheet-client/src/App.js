import './App.css';
import TableCell from './TableCell';
import FormulaBar from './FormulaBar';
import {Rect} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

import {GetCellsRequest, InsertCell, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import React, {Component} from 'react';

let hostname = "https://spreadsheet.yellowpapersun.net"
// let hostname = 'http://localhost:8080';
let apiClient = new SpreadsheetAPIClient(hostname,
                               null, null);

class App extends React.Component {

  constructor(props) {
      super(props);

      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleFormulaBarChanged = this.handleFormulaBarChanged.bind(this);
      this.handleTableCellChanged = this.handleTableCellChanged.bind(this);
      this.handleTableCellFocused = this.handleTableCellFocused.bind(this);
      this.handleCellInsert = this.handleCellInsert.bind(this);

      const items = [];
      for (let i = 0; i < this.props.numRows; i++) {
          const cells = [];
          for (let j = 0; j < this.props.numCols; j++) {
              let cell = <TableCell key={i*this.props.numCols + j}
                         onChanged={this.handleTableCellChanged} onFocus={this.handleTableCellFocused} cell={{row: i, col: j, value:"",
                          displayValue: ""}} tableRef={React.createRef()} onKeyDown={this.handleCellInsert} />;
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
      tableCell.props = {...tableCell.props, cell: {...tableCell.props.cell, value: c.getValue(), displayValue: c.getDisplayValue()}};
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
          this.insertIntoTable(response.getCellsList());
      }
    });
  }

  handleCellInsert(cell) {
    console.log("insert cell: ", cell);
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
          this.insertIntoTable(response.getCellsList());
      }
    });
  }

  handleTableCellFocused(row, col, textContent) {
      console.log("handling focus", row, col);
      let table = [...this.state.table];
      if (this.state.currSelection !== undefined) {
          let currIdx = this.state.currSelection.row *
            this.props.numCols + this.state.currSelection.col
          let newCell = table[currIdx];
          let newProps = {...newCell.props, selected: false};
          table[currIdx] = {...newCell, props: newProps};
          console.log("cleared selection for idx", currIdx);
      }

      let idx = row * this.props.numCols + col
      console.log(idx);
      let tableCell = table[idx];
      let newProps = {...tableCell.props, selected: true};
      table[idx] = {...tableCell, props: newProps};
      this.setState({
          table: table,
          currSelection: {row: row, col: col},
          selectedCell: {
            row: row,
            col: col,
            value: textContent
          }
      });
      table[idx].props.tableRef.current.focus();
  }

  handleFormulaBarChanged(value) {
      let selectedCell = {...this.state.selectedCell, value: value};
      this.setState({selectedCell: selectedCell});
  }

  handleTableCellChanged(targetCell, textContent) {
      let cell = {...targetCell, value: textContent};
      let selectedCell = {...this.state.selectedCell, value: textContent};
      this.setState({selectedCell: selectedCell});
  }

  render() {
    let items = [];
    let header_items = [<td class="TableCell"> </td>];
    for (let i = 0; i < this.props.numCols; i++) {
        header_items.push(<td class="TableCell">{ String.fromCharCode('A'.charCodeAt() + i) }</td>);
    }

    for (let i = 0; i < this.props.numRows; i++) {
        items.push(
            <tr>
            <td class="TableCell">{ (i + 1).toString()}</td>
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
