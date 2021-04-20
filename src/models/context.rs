pub trait EvalContext {
    fn get_cell(&self, row: i32, col: i32) -> super::Cell;
    // fn get_cells(&self, rect: models::Rect) -> Vec<models::Cell>;
}
