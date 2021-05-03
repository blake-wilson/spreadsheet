#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub value: String,
    pub display_value: String,
}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub struct CellLocation {
    pub row: i32,
    pub col: i32,
}

impl CellLocation {
    pub fn to_range(&self) -> CellRange {
        CellRange {
            start_row: self.row,
            stop_row: self.row + 1,
            start_col: self.col,
            stop_col: self.col + 1,
        }
    }
}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
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

    pub fn loc(&self) -> CellLocation {
        CellLocation {
            row: self.row,
            col: self.col,
        }
    }

    pub fn to_range(&self) -> CellRange {
        CellRange {
            start_row: self.row,
            stop_row: self.row + 1,
            start_col: self.col,
            stop_col: self.col + 1,
        }
    }
}
