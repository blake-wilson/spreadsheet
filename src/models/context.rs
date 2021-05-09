pub trait EvalContext {
    fn get_cell(&self, row: i32, col: i32) -> super::Cell;
    fn get_cells(&self, rect: super::Rect) -> Vec<super::Cell>;
    fn num_rows(&self) -> i32;
    fn num_cols(&self) -> i32;
}
