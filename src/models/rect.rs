#[derive(Debug, PartialEq)]
pub struct Rect {
    pub start_row: i32,
    pub start_col: i32,
    pub stop_row: i32,
    pub stop_col: i32,
}

impl Rect {
    pub fn clamp(&self, max_row: i32, max_col: i32) -> Rect {
        Rect {
            start_row: self.start_row.clamp(0, max_row),
            stop_row: self.stop_row.clamp(0, max_row),
            start_col: self.start_col.clamp(0, max_col),
            stop_col: self.stop_col.clamp(0, max_col),
        }
    }
}

pub fn width(r: &Rect) -> i32 {
    r.stop_col - r.start_col
}
