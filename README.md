## Spreadsheet
This project defines a simple Spreadsheet application for running computations
on tabular data.

The server is written in Rust and exposes a gRPC API. Clients can use this API
to enter data into the spreadsheet and read data out of it.

The bundled client is written in Javascript (with React) and is suitable for web deployment.

The documentation for the API can be found [here](src/proto/grpc/doc/).

The application is currently deployed [here](https://yellowpapersun.com/projects/spreadsheet).
