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
const tableId = "test-table-id";

class App extends React.Component {

  constructor(props) {
      super(props);

      this.handleKeyDown = this.handleKeyDown.bind(this);
      this.handleFormulaBarChanged = this.handleFormulaBarChanged.bind(this);
      this.handleTableCellChanged = this.handleTableCellChanged.bind(this);
      this.handleTableCellFocused = this.handleTableCellFocused.bind(this);
      this.handleCellInsert = this.handleCellInsert.bind(this);
      this.handleTableCellSubmit = this.handleTableCellSubmit.bind(this);
      this.focusAt = this.focusAt.bind(this);
      this.handleFormulaBarSubmit = this.handleFormulaBarSubmit.bind(this);

      const items = [];
      for (let i = 0; i < this.props.numRows; i++) {
          const cells = [];
          for (let j = 0; j < this.props.numCols; j++) {
              let cell = <TableCell key={i*this.props.numCols + j}
                         onChanged={this.handleTableCellChanged} onFocus={this.handleTableCellFocused} cell={{row: i, col: j, value:"",
                          displayValue: ""}} tableRef={React.createRef()} onKeyDown={this.handleTableCellSubmit} />;
              items.push(cell);
          }
      }
      this.state = {table: items, numRows: this.props.numRows, numCols: this.props.numCols};

      // get the initial cells values
      var request = new GetCellsRequest();
      request.setTableid(tableId);
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
      let tableCell = {...table[idx]};
      tableCell.props = {...tableCell.props, cell: {row:row, col: col, value: c.getValue(), displayValue: c.getDisplayValue()}};
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
    request.setTableid(tableId);
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

  handleTableCellSubmit(cell, newValue) {
      if (newValue !== undefined && (newValue !== cell.value)) {
        let toInsert = {...cell, value: newValue};
        this.handleCellInsert(toInsert);
      }
      this.focusAt(cell.row + 1, cell.col);
  }

  handleFormulaBarSubmit(cell) {
      this.handleCellInsert(cell);
      this.focusAt(cell.row + 1, cell.col);
  }

  handleCellInsert(cell) {
    var c1 = new InsertCell();
    c1.setRow(cell.row);
    c1.setCol(cell.col);
    c1.setValue(cell.value);
    var request = new InsertCellsRequest();
    request.setTableid(tableId);
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
      let table = [...this.state.table];
      if (this.state.currSelection !== undefined) {
          let currIdx = this.state.currSelection.row *
            this.props.numCols + this.state.currSelection.col
          let newCell = table[currIdx];
          let newProps = {...newCell.props, selected: false};
          table[currIdx] = {...newCell, props: newProps};
      }

      let idx = row * this.props.numCols + col
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
  }

  focusAt(row, col) {
      let idx = row * this.props.numCols + col
      this.state.table[idx].props.tableRef.current.focus();
  }

  handleFormulaBarChanged(value) {
      let selectedCell = {...this.state.selectedCell, value: value};
      this.setState({selectedCell: selectedCell});

      let table = [...this.state.table];
      let idx = selectedCell.row * this.props.numCols + selectedCell.col
      let tableCell = table[idx];
      let insertCell = tableCell.props.cell;
      let newProps = {...tableCell.props, cell: insertCell, enteredValue: value };
      table[idx] = {...tableCell, props: newProps};
      this.setState({
          table: table,
      });
  }

  handleTableCellChanged(targetCell, textContent) {
      let cell = targetCell;
      let selectedCell = {...this.state.selectedCell, value: textContent};
      this.setState({selectedCell: selectedCell});

      let table = [...this.state.table];
      let idx = cell.row * this.props.numCols + cell.col
      let tableCell = table[idx];
      let insertCell = tableCell.props.cell;
      let newProps = {...tableCell.props, cell: insertCell, enteredValue: textContent };
      table[idx] = {...tableCell, props: newProps};
      this.setState({
          table: table,
      });
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
                            onSubmit={this.handleFormulaBarSubmit} />
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
