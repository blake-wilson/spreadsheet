mod models;
mod parser;

use proto::grpc::SpreadsheetApi;

#[derive(Clone)]
struct SpreadsheetService;

impl SpreadsheetAPI for SpreadsheetService {}

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
