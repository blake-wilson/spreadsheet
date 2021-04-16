#[derive(Debug, PartialEq)]
pub struct Rect {
    pub start_row: i32,
    pub start_col: i32,
    pub stop_row: i32,
    pub stop_col: i32,
}

pub fn width(r: &Rect) -> i32 {
    r.stop_col - r.start_col
}
