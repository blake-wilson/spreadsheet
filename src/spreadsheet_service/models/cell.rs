#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub value: String,
    pub display_value: String,
}

#[derive(Debug, Eq, PartialOrd, Ord, Copy, Hash, Clone, PartialEq)]
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

    // returns the magnitude of the cell location as if it were a vector
    pub fn magnitude(&self) -> i32 {
        return self.row * self.row + self.col * self.col;
    }
}

#[derive(Debug, Eq, Copy, Hash, Clone, PartialEq)]
pub struct CellRange {
    pub start_row: i32,
    pub start_col: i32,
    pub stop_row: i32,
    pub stop_col: i32,
}

impl CellRange {
    pub fn clamp(&mut self, max_rows: i32) {
        if self.stop_row != -1 {
            return;
        }
        self.stop_row = max_rows
    }
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

    pub fn empty() -> Cell {
        Cell {
            row: 0,
            col: 0,
            value: "".to_string(),
            display_value: "".to_string(),
        }
    }
    pub fn new(row: i32, col: i32, value: String) -> Cell {
        Cell {
            row,
            col,
            value: value.clone(),
            display_value: String::from(""),
        }
    }
}
