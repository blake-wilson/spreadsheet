#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub value: String,
    pub display_value: String,
}

impl Cell {
    pub fn is_formula(&self) -> bool {
        self.value.len() > 0 && self.value.as_bytes()[0] as char == '='
    }
}
