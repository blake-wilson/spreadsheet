mod models;
mod parser;

#[path = "proto/grpc/api.rs"]
mod api;
#[path = "proto/grpc/api_grpc.rs"]
mod api_grpc;

#[derive(Clone)]
struct SpreadsheetService;

// impl api::SpreadsheetApi for SpreadsheetService {}

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
