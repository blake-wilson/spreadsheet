use super::super::models;
use super::super::models::context::EvalContext;
use super::super::parser;
use super::formula_graph::FormulaGraph;

pub trait CellsService {
    // insert_cells inserts the provided list of cells into the store.
    fn insert_cells(
        &mut self,
        cells: &Vec<models::Cell>,
    ) -> Result<Vec<models::Cell>, parser::Error>;

    // get_cells returns a Vector of cells in the provided rectangle
    fn get_cells(&self, r: models::Rect) -> Vec<models::Cell>;
}

pub struct MemoryCellsService {
    num_rows: i32,
    num_cols: i32,
    // data stored in row-major order
    data: Vec<models::Cell>,

    formula_graph: FormulaGraph,
}

impl EvalContext for MemoryCellsService {
    fn get_cell(&self, row: i32, col: i32) -> Option<models::Cell> {
        self.get_cell(row, col)
    }

    fn get_cells(&self, rect: models::Rect) -> Vec<models::Cell> {
        <_ as CellsService>::get_cells(self, rect)
    }

    fn num_rows(&self) -> i32 {
        self.num_rows
    }

    fn num_cols(&self) -> i32 {
        self.num_cols
    }
}

impl CellsService for MemoryCellsService {
    fn insert_cells(
        &mut self,
        cells: &Vec<models::Cell>,
    ) -> Result<Vec<models::Cell>, parser::Error> {
        let mut ret_cells = vec![];

        for c in cells {
            self.set_cell(&c);
        }

        // Recalculate after inserting values for all cells
        for c in cells {
            let mut cc = self.get_cell(c.row, c.col).unwrap();

            // Update the formula graph and recompute necessary cells
            let formula = parser::parse(&cc.value)?;
            let mut refs = parser::get_refs(&formula);
            refs.iter_mut().for_each(|r| (*r).clamp(self.num_rows));
            let mut insert_res = self.formula_graph.insert_cell(cc.clone(), refs);
            if !insert_res.circular {
                while let Some(c) = insert_res.inserted_cells.pop() {
                    // We don't need to check refs again here since the formula graph already computed
                    // all the required re-evals.
                    let mut eval_cell = self.get_cell(c.row, c.col).unwrap().clone();
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
            } else {
                while let Some(c) = insert_res.inserted_cells.pop() {
                    println!("circular cell\n");
                    let display_value = "#CIRCULAR!".to_owned();
                    let mut eval_cell = self.get_cell(c.row, c.col).unwrap().clone();
                    eval_cell.display_value = display_value;
                    self.set_cell(&eval_cell);
                    ret_cells.push(eval_cell);
                }
            }
        }
        Ok(ret_cells)
    }

    fn get_cells(&self, r: models::Rect) -> Vec<models::Cell> {
        let mut result_cells: Vec<models::Cell> = Vec::new();
        let clamped = r.clamp(self.num_rows, self.num_cols);
        for row in clamped.start_row..clamped.stop_row {
            for col in clamped.start_col..clamped.stop_col {
                let c = self.get_cell(row, col);
                if !c.is_none() {
                    result_cells.push(c.unwrap().clone());
                }
            }
        }
        result_cells
    }
}

impl MemoryCellsService {
    pub fn new(num_rows: i32, num_cols: i32) -> Self {
        MemoryCellsService {
            num_cols,
            num_rows,
            data: vec![models::Cell::empty(); (num_cols * num_rows) as usize],
            formula_graph: FormulaGraph::new(),
        }
    }
    pub fn get_cell(&self, row: i32, col: i32) -> Option<models::Cell> {
        let c = self.data[row_major_idx(row, col, self.num_cols) as usize].clone();
        match c.value.as_ref() {
            "" => None,
            _ => Some(c),
        }
    }
    pub fn set_cell(&mut self, cell: &models::Cell) {
        self.data[row_major_idx(cell.row, cell.col, self.num_cols) as usize] = cell.clone();
    }
}

fn row_major_idx(row: i32, col: i32, num_cols: i32) -> i32 {
    (row * num_cols) + col
}
