use spreadsheet_service::{modeler, parser, service};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};
use service::CellsService;
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::{io, thread};

#[path = "proto/grpc/api.rs"]
mod api;
#[path = "proto/grpc/api_grpc.rs"]
mod api_grpc;

#[derive(Clone)]
struct SpreadsheetService {
    cells_service: Arc<RwLock<HashMap<String, service::MemoryCellsService>>>,
}

impl SpreadsheetService {
    fn create_table_if_not_exists(&mut self, table_id: &str) {
        self.cells_service
            .write()
            .unwrap()
            .entry(table_id.to_owned())
            .or_insert(service::MemoryCellsService::new(50, 26));
    }
}

impl SpreadsheetService {
    fn insert_cells(
        &mut self,
        req: api::InsertCellsRequest,
    ) -> Result<api::InsertCellsResponse, ()> {
        let insert_res: Result<Vec<models::Cell>, parser::Error>;
        {
            let cells = insert_cells_to_models(req.get_cells());
            self.create_table_if_not_exists(req.get_tableId());

            let cs = &mut self.cells_service.write().unwrap();
            let service = cs.get_mut(req.get_tableId()).unwrap();

            insert_res = service.insert_cells(&cells);
        }
        let mut resp = api::InsertCellsResponse::default();
        match insert_res {
            Ok(inserted_cells) => {
                resp.set_cells(protobuf::RepeatedField::from_vec(model_cells_to_api(
                    inserted_cells,
                )));
                Ok(resp)
            }
            Err(e) => {
                println!("error inserting cells: {:?}", e);
                Err(())
            }
        }
    }

    fn get_cells(&mut self, req: api::GetCellsRequest) -> Result<api::GetCellsResponse, ()> {
        let rect = api_rect_to_model(req.get_rect());
        let cells: Vec<models::Cell>;
        {
            self.create_table_if_not_exists(req.get_tableId());
            let cs = &mut self.cells_service.read().unwrap();
            let service = cs.get(req.get_tableId()).unwrap();
            cells = service.get_cells(rect);
        }
        let mut resp = api::GetCellsResponse::default();
        resp.set_cells(protobuf::RepeatedField::from_vec(model_cells_to_api(cells)));
        Ok(resp)
    }
}

impl api_grpc::SpreadsheetApi for SpreadsheetService {
    fn insert_cells(
        &mut self,
        ctx: RpcContext<'_>,
        req: api::InsertCellsRequest,
        sink: UnarySink<api::InsertCellsResponse>,
    ) {
        let resp = SpreadsheetService::insert_cells(self, req).unwrap();
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply: {:?}", e))
            .map(|_| ());
        ctx.spawn(f);
    }

    fn get_cells(
        &mut self,
        ctx: RpcContext<'_>,
        req: api::GetCellsRequest,
        sink: UnarySink<api::GetCellsResponse>,
    ) {
        let resp = SpreadsheetService::get_cells(self, req).unwrap();
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply: {:?}", e))
            .map(|_| ());
        ctx.spawn(f);
    }
}

fn main() {
    let ss_service = SpreadsheetService {
        cells_service: Arc::new(RwLock::new(HashMap::new())),
    };

    let env = Arc::new(Environment::new(1));
    let service = api_grpc::create_spreadsheet_api(ss_service);
    let quota = ResourceQuota::new(Some("SpreadsheetServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server
        .add_listening_port("0.0.0.0:9090", grpcio::ServerCredentials::insecure())
        .unwrap();
    server.start();
    // for (host, port) in server.bind_addrs() {
    //     println!("listening on {}:{}", host, port);
    // }
    let (tx, rx): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
    // thread::spawn(move || {
    //     println!("Press ENTER to exit...");
    //     let _ = io::stdin().read(&mut [0]).unwrap();
    //     tx.send(()).unwrap();
    // });
    let _ = block_on(rx);
    // let _ = block_on(server.shutdown());
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
