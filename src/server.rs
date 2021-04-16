mod models;
mod parser;

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

#[path = "proto/grpc/api.rs"]
mod api;
#[path = "proto/grpc/api_grpc.rs"]
mod api_grpc;

mod service;

#[derive(Clone)]
struct SpreadsheetService;

impl api_grpc::SpreadsheetApi for SpreadsheetService {
    fn insert_cells(
        &mut self,
        ctx: RpcContext<'_>,
        req: api::InsertCellsRequest,
        sink: UnarySink<api::InsertCellsResponse>,
    ) {
        println!("got request");
        for c in req.get_cells() {
            println!(
                "inserting cell {:?} at row {:?} and col {:?}",
                c.get_value(),
                c.get_col(),
                c.get_row()
            );
        }
        let mut resp = api::InsertCellsResponse::default();
        resp.set_numInserted(req.get_cells().len() as i32);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f);
    }
}

fn main() {
    let mut tokens = parser::lex("CALL(10 * 3, CALLB(100, 200))");
    let tree = parser::parse(&mut tokens);
    println!("tree: {:?}", tree);
    let c = models::Cell {
        row: 10,
        col: 30,
        value: "10".to_string(),
        display_value: "10".to_string(),
    };
    println!("cell: {:?}", c);

    let env = Arc::new(Environment::new(1));
    let service = api_grpc::create_spreadsheet_api(SpreadsheetService);
    let quota = ResourceQuota::new(Some("SpreadsheetServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 9090)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(()).unwrap();
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
