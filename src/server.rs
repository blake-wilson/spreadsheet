mod models;
mod parser;
mod service;

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};
use service::CellsService;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{io, thread};

#[path = "proto/grpc/api.rs"]
mod api;
#[path = "proto/grpc/api_grpc.rs"]
mod api_grpc;

#[derive(Clone)]
struct SpreadsheetService {
    cells_service: Arc<Mutex<service::MemoryCellsService>>,
}

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
                c.get_row(),
                c.get_col()
            );
        }
        let inserted_cells: Vec<models::Cell>;
        {
            let cells = insert_cells_to_models(req.get_cells());
            let mut cs = self.cells_service.lock().unwrap();
            inserted_cells = cs.insert_cells(&cells).unwrap();
            println!("inserted cells: {:?}", inserted_cells);
        }
        let mut resp = api::InsertCellsResponse::default();
        resp.set_cells(protobuf::RepeatedField::from_vec(model_cells_to_api(
            inserted_cells,
        )));
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f);
    }
    fn get_cells(
        &mut self,
        ctx: RpcContext<'_>,
        req: api::GetCellsRequest,
        sink: UnarySink<api::GetCellsResponse>,
    ) {
        let rect = api_rect_to_model(req.get_rect());
        let cells: Vec<models::Cell>;
        {
            let cs = self.cells_service.lock().unwrap();
            cells = cs.get_cells(rect);
        }
        let mut resp = api::GetCellsResponse::default();
        resp.set_cells(protobuf::RepeatedField::from_vec(model_cells_to_api(cells)));

        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f);
    }
}

fn main() {
    let tree = parser::parse("=CALL(10 * 3, CALLB(100, 200))");
    println!("tree: {:?}", tree);
    let c = models::Cell {
        row: 10,
        col: 30,
        value: "10".to_string(),
        display_value: "10".to_string(),
    };
    println!("cell: {:?}", c);

    let cells_service = service::MemoryCellsService::new(50, 26);
    let ss_service = SpreadsheetService {
        cells_service: Arc::new(Mutex::new(cells_service)),
    };

    let env = Arc::new(Environment::new(1));
    let service = api_grpc::create_spreadsheet_api(ss_service);
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

fn insert_cells_to_models(insert_cells: &[api::InsertCell]) -> Vec<models::Cell> {
    let mut ret = vec![];
    for c in insert_cells {
        ret.push(models::Cell {
            row: c.row,
            col: c.col,
            value: c.value.clone(),
            display_value: "".to_owned(),
        });
    }
    ret
}

fn model_cells_to_api(cells: Vec<models::Cell>) -> Vec<api::Cell> {
    let mut ret = vec![];
    for c in cells {
        let mut api_cell = api::Cell::default();
        api_cell.set_row(c.row);
        api_cell.set_col(c.col);
        api_cell.set_value(c.value);
        api_cell.set_display_value(c.display_value);
        ret.push(api_cell);
    }
    ret
}

fn api_rect_to_model(rect: &api::Rect) -> models::Rect {
    models::Rect {
        start_row: rect.start_row,
        start_col: rect.start_col,
        stop_row: rect.stop_row,
        stop_col: rect.stop_col,
    }
}
