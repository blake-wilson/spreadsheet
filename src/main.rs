mod models;
mod parser;

use futures::prelude::*;
use grpcio::{RpcContext, UnarySink};

#[path = "proto/grpc/api.rs"]
mod api;
#[path = "proto/grpc/api_grpc.rs"]
mod api_grpc;

#[derive(Clone)]
struct SpreadsheetService;

impl api_grpc::SpreadsheetApi for SpreadsheetService {
    fn insert_cells(
        &mut self,
        ctx: RpcContext<'_>,
        req: api::InsertCellsRequest,
        sink: UnarySink<api::InsertCellsResponse>,
    ) {
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
    };
    println!("cell: {:?}", c);
}
