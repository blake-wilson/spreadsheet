use super::super::models;
use super::super::models::context::EvalContext;
use super::super::parser;

pub trait CellsService {
    // insert_cells inserts the provided list of cells into the store.
    fn insert_cells(
        &mut self,
        cells: &Vec<models::Cell>,
    ) -> Result<Vec<models::Cell>, &'static str>;

    // get_cells returns a Vector of cells in the provided rectangle
    fn get_cells(&self, r: models::Rect) -> Vec<models::Cell>;
}

pub struct MemoryCellsService {
    num_cols: i32,
    // data stored in row-major order
    data: Vec<models::Cell>,
}

impl EvalContext for MemoryCellsService {
    fn get_cell(&self, row: i32, col: i32) -> models::Cell {
        println!("getting cell at row {} col {}", row, col);
        self.data
            .get((row * self.num_cols + col) as usize)
            .unwrap()
            .clone()
    }
}

impl CellsService for MemoryCellsService {
    fn insert_cells(
        &mut self,
        cells: &Vec<models::Cell>,
    ) -> Result<Vec<models::Cell>, &'static str> {
        let mut ret_cells = vec![];

        for c in cells {
            let mut cc = c.clone();
            let mut tokens = parser::lex(&c.value);
            let formula = parser::parse(&mut tokens)?;
            let display_value = parser::evaluate(formula, self);
            cc.display_value = display_value;

            let idx = cc.row * self.num_cols + cc.col;
            let curr_cell = self.data.get(idx as usize).unwrap();

            if curr_cell.value != cc.value || curr_cell.display_value != cc.display_value {
                // Only insert if we are updating value or value has been recomputed
                ret_cells.push(cc.clone());
                self.data
                    .insert((cc.row * self.num_cols + cc.col) as usize, cc);
            }
        }
        Ok(ret_cells)
    }
    fn get_cells(&self, r: models::Rect) -> Vec<models::Cell> {
        let mut result_cells: Vec<models::Cell> = Vec::new();
        for row in r.start_row..r.stop_row {
            let start_idx = row * self.num_cols;
            let stop_idx = start_idx + models::rect::width(&r);
            for idx in start_idx..stop_idx {
                let c = self.data.get(idx as usize).unwrap().clone();
                result_cells.push(c);
            }
        }
        println!("got cells {:?}", result_cells);
        result_cells
    }
}

impl MemoryCellsService {
    pub fn new(num_rows: i32, num_cols: i32) -> Self {
        MemoryCellsService {
            num_cols: 26,
            data: vec![
                models::Cell {
                    row: -1,
                    col: -1,
                    value: "".to_string(),
                    display_value: "".to_string(),
                };
                (num_cols * num_rows) as usize
            ],
        }
    }
}
