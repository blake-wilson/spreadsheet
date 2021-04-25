#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub value: String,
    pub display_value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellRange {
    pub start_row: i32,
    pub start_col: i32,
    pub stop_row: i32,
    pub stop_col: i32,
}

impl Cell {
    pub fn is_formula(&self) -> bool {
        self.value.len() > 0 && self.value.as_bytes()[0] as char == '='
    }
}
