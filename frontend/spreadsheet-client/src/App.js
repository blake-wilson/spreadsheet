import logo from './logo.svg';
import './App.css';
import TableCell from './TableCell';

import {InsertCell, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import React, {Component} from 'react';


class App extends React.Component {

  constructor(props) {
      super(props);
      this.state = {table: {}};
  }


  render() {
    const items = [];
    for (let i = 0; i < this.props.numRows; i++) {
        const cells = [];
        for (let j = 0; j < this.props.numCols; j++) {
            console.log("i:",i, "j", j);
            cells.push(<TableCell row={i} col={j} onKeyDown={this.handleKeyDown} />);
        }
        items.push(<tr>{cells}</tr>);
    }
    return (
      <div className="App">
        <header className="App-header">
          <table border="1px solid white"> 
              { items }
          </table>
        </header>
      </div>
    );
  }
}

export default App;
