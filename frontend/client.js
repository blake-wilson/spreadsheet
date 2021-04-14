const {InsertCellsRequest} = require('./api_pb.js');
const {SpreadsheetAPIClient} = require('./api_grpc_web_pb.js');

var client = new SpreadsheetAPIClient('http://' + window.location.hostname + ':8080',
                               null, null);

// simple unary call
var cell1 = new spreadsheet.InsertCell();
cell1.setRow(10);
cell1.setCol(5);
cell1.setValue("this is the value");
var request = new InsertCellsRequest();
request.setCellsList([c1]);

client.insertCells(request, {}, (err, response) => {
  if (err) {
    console.log(`Unexpected error for sayHello: code = ${err.code}` +
                `, message = "${err.message}"`);
  } else {
    console.log(response.getMessage());
  }
});

