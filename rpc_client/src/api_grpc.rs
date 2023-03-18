// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SPREADSHEET_API_INSERT_CELLS: ::grpcio::Method<super::api::InsertCellsRequest, super::api::InsertCellsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/spreadsheet.SpreadsheetAPI/InsertCells",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPREADSHEET_API_GET_CELLS: ::grpcio::Method<super::api::GetCellsRequest, super::api::GetCellsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/spreadsheet.SpreadsheetAPI/GetCells",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SpreadsheetApiClient {
    client: ::grpcio::Client,
}

impl SpreadsheetApiClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SpreadsheetApiClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn insert_cells_opt(&self, req: &super::api::InsertCellsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::InsertCellsResponse> {
        self.client.unary_call(&METHOD_SPREADSHEET_API_INSERT_CELLS, req, opt)
    }

    pub fn insert_cells(&self, req: &super::api::InsertCellsRequest) -> ::grpcio::Result<super::api::InsertCellsResponse> {
        self.insert_cells_opt(req, ::grpcio::CallOption::default())
    }

    pub fn insert_cells_async_opt(&self, req: &super::api::InsertCellsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::InsertCellsResponse>> {
        self.client.unary_call_async(&METHOD_SPREADSHEET_API_INSERT_CELLS, req, opt)
    }

    pub fn insert_cells_async(&self, req: &super::api::InsertCellsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::InsertCellsResponse>> {
        self.insert_cells_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cells_opt(&self, req: &super::api::GetCellsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::GetCellsResponse> {
        self.client.unary_call(&METHOD_SPREADSHEET_API_GET_CELLS, req, opt)
    }

    pub fn get_cells(&self, req: &super::api::GetCellsRequest) -> ::grpcio::Result<super::api::GetCellsResponse> {
        self.get_cells_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cells_async_opt(&self, req: &super::api::GetCellsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::GetCellsResponse>> {
        self.client.unary_call_async(&METHOD_SPREADSHEET_API_GET_CELLS, req, opt)
    }

    pub fn get_cells_async(&self, req: &super::api::GetCellsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::GetCellsResponse>> {
        self.get_cells_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SpreadsheetApi {
    fn insert_cells(&mut self, ctx: ::grpcio::RpcContext, req: super::api::InsertCellsRequest, sink: ::grpcio::UnarySink<super::api::InsertCellsResponse>);
    fn get_cells(&mut self, ctx: ::grpcio::RpcContext, req: super::api::GetCellsRequest, sink: ::grpcio::UnarySink<super::api::GetCellsResponse>);
}

pub fn create_spreadsheet_api<S: SpreadsheetApi + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPREADSHEET_API_INSERT_CELLS, move |ctx, req, resp| {
        instance.insert_cells(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SPREADSHEET_API_GET_CELLS, move |ctx, req, resp| {
        instance.get_cells(ctx, req, resp)
    });
    builder.build()
}
