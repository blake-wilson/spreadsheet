#[macro_use]
extern crate log;

#[path = "log_util.rs"]
mod log_util;

#[path = "proto/grpc/api.rs"]
mod api;
#[path = "proto/grpc/api_grpc.rs"]
mod api_grpc;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:9090");
    let client = api_grpc::SpreadsheetApiClient::new(ch);

    let mut req = api::InsertCellsRequest::default();
    let mut c1 = api::InsertCell::default();
    c1.set_row(1);
    c1.set_col(2);
    c1.set_value("this is the value".to_owned());
    req.set_cells(protobuf::RepeatedField::from(vec![c1]));
    let reply = client.insert_cells(&req).expect("rpc");
    info!("cells inserted: {:?}", reply.get_cells());
}
