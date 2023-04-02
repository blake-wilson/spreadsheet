mod models;
mod parser;
mod service;

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
