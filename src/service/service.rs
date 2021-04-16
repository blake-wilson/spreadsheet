use super::super::models;
use super::super::parser;

pub trait CellsService {
    // insert_cells inserts the provided list of cells into the store.
    fn insert_cells(&mut self, cells: Vec<models::Cell>);

    // get_cells returns a Vector of cells in the provided rectangle
    fn get_cells(&mut self, r: models::Rect) -> Vec<models::Cell>;
}

struct Service {
    num_cols: i32,
    // data stored in row-major order
    data: Vec<models::Cell>,
}

impl CellsService for Service {
    fn insert_cells(&mut self, cells: Vec<models::Cell>) {
        for c in cells {
            self.data
                .insert((c.row * self.num_cols + c.col) as usize, c);
        }
    }
    fn get_cells(&mut self, r: models::Rect) -> Vec<models::Cell> {
        let mut result_cells: Vec<models::Cell> = Vec::new();
        for row in r.start_row..r.stop_row {
            let start_idx = row * self.num_cols;
            let stop_idx = start_idx + models::rect::width(&r);
            for idx in start_idx..stop_idx {
                let mut c = self.data.get(idx as usize).unwrap().clone();
                let mut tokens = parser::lex(&c.value);
                let formula = parser::parse(&mut tokens);
                let display_value = parser::evaluate(formula);
                c.display_value = display_value;
                result_cells.push(c);
            }
        }
        result_cells
    }
}
