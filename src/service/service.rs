use super::super::models;
use super::super::models::context::EvalContext;
use super::super::parser;
use super::formula_graph::FormulaGraph;

pub trait CellsService {
    // insert_cells inserts the provided list of cells into the store.
    fn insert_cells(&mut self, cells: &Vec<models::Cell>) -> Result<Vec<models::Cell>, String>;

    // get_cells returns a Vector of cells in the provided rectangle
    fn get_cells(&self, r: models::Rect) -> Vec<models::Cell>;
}

pub struct MemoryCellsService {
    num_cols: i32,
    // data stored in row-major order
    data: Vec<models::Cell>,

    formula_graph: FormulaGraph,
}

impl EvalContext for MemoryCellsService {
    fn get_cell(&self, row: i32, col: i32) -> models::Cell {
        self.get_cell(row, col)
    }

    fn get_cells(&self, rect: models::Rect) -> Vec<models::Cell> {
        <_ as CellsService>::get_cells(self, rect)
    }
}

impl CellsService for MemoryCellsService {
    fn insert_cells(&mut self, cells: &Vec<models::Cell>) -> Result<Vec<models::Cell>, String> {
        let mut ret_cells = vec![];

        for c in cells {
            self.set_cell(&c);
        }

        // Recalculate after inserting values for all cells
        for c in cells {
            let mut cc = self.get_cell(c.row, c.col);

            // Update the formula graph and recompute necessary cells
            let formula = parser::parse(&cc.value)?;
            let refs = parser::get_refs(&formula);
            println!("refs: {:?}", refs);
            let mut to_eval = self.formula_graph.insert_cell(cc.clone(), refs);
            println!("to re-evaluate: {:?}", to_eval);

            while let Some(c) = to_eval.pop() {
                // We don't need to check refs again here since the formula graph already computed
                // all the required re-evals.
                let mut eval_cell = self.get_cell(c.row, c.col).clone();
                let formula = parser::parse(&eval_cell.value)?;
                let display_value = parser::evaluate(formula, self);
                eval_cell.display_value = display_value;
                self.set_cell(&eval_cell);
                ret_cells.push(eval_cell);
            }

            let display_value = parser::evaluate(formula, self);
            if display_value != cc.display_value {
                cc.display_value = display_value;
            }
            self.set_cell(&cc);
            ret_cells.push(cc);
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
        result_cells
    }
}

impl MemoryCellsService {
    pub fn new(num_rows: i32, num_cols: i32) -> Self {
        MemoryCellsService {
            num_cols,
            data: vec![
                models::Cell {
                    row: -1,
                    col: -1,
                    value: "".to_string(),
                    display_value: "".to_string(),
                };
                (num_cols * num_rows) as usize
            ],
            formula_graph: FormulaGraph::new(),
        }
    }
    pub fn get_cell(&self, row: i32, col: i32) -> models::Cell {
        self.data[row_major_idx(row, col, self.num_cols) as usize].clone()
    }

    pub fn set_cell(&mut self, cell: &models::Cell) {
        self.data[row_major_idx(cell.row, cell.col, self.num_cols) as usize] = cell.clone();
    }
}

fn row_major_idx(row: i32, col: i32, num_cols: i32) -> i32 {
    (row * num_cols) + col
}
