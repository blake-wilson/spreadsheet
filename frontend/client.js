const {InsertCellsRequest} = require('./spreadsheet_api_pb.js');
const {SpreadsheetAPIClient} = require('./spreadsheet_api_grpc_web_pb.js');

var client = new SpreadsheetApiClient('http://' + window.location.hostname + ':8080',
                               null, null);

// simple unary call
var request = new InsertCellsRequest();
request.setCells([{
    row: 10,
    col: 5,
    value: "this is the value",
  }, {
    row: 1,
    col: 1,
    value: "this is another value",
  }
]);

client.insertCells(request, {}, (err, response) => {
  if (err) {
    console.log(`Unexpected error for sayHello: code = ${err.code}` +
                `, message = "${err.message}"`);
  } else {
    console.log(response.getMessage());
  }
});

