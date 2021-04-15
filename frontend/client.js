const {InsertCellsRequest} = require('./api_pb.js');
const {SpreadsheetAPIClient} = require('./api_grpc_web_pb.js');

var client = new SpreadsheetAPIClient('http://' + window.location.hostname + ':8080',
                               null, null);

// simple unary call
var c1 = new proto.spreadsheet.InsertCell();
c1.setRow(10);
c1.setCol(5);
c1.setValue("this is the value");
var request = new proto.spreadsheet.InsertCellsRequest();
request.setCellsList([c1]);

client.insertCells(request, {}, (err, response) => {
  if (err) {
    console.log(`Unexpected error for sayHello: code = ${err.code}` +
                `, message = "${err.message}"`);
  } else {
    console.log("inserted " + response.getNuminserted().toString() + " cells");
  }
});

