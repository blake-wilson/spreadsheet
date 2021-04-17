import logo from './logo.svg';
import './App.css';

import {InsertCell, InsertCellsRequest, InsertCellsResponse} from './api_pb.js';
import {SpreadsheetAPIClient} from './api_grpc_web_pb.js';

let apiClient = new SpreadsheetAPIClient('http://' + window.location.hostname + ':8080',
                               null, null);

function App(props) {
  const items = [];
  for (let i = 0; i < props.numRows; i++) {
      const cells = [];
      for (let j = 0; j < props.numCols; j++) {
          cells.push(<td contenteditable='true'
              height="20px" width="72px"></td>);
      }
      items.push(<tr>{cells}</tr>);
  }
  var c1 = new InsertCell();
  c1.setRow(10);
  c1.setCol(5);
  c1.setValue("5+7");
  var request = new InsertCellsRequest();
  request.setCellsList([c1]);
  apiClient.insertCells(request, {}, (err, response) => {
  if (err) {
    console.log(`Unexpected error for sayHello: code = ${err.code}` +
                `, message = "${err.message}"`);
  } else {
      console.log("inserted " + response.getNumInserted() + " cells");
  }
});
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

export default App;
