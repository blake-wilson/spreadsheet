use glib::Object;
use gtk::glib;
use rpc_client::api;
mod imp;

const NUM_COLS: i32 = 10;
const NUM_ROWS: i32 = 20;

// ANCHOR: glib_wrapper_and_new
glib::wrapper! {
    pub struct SpreadsheetCellObject(ObjectSubclass<imp::SpreadsheetCellObject>);
}

impl SpreadsheetCellObject {
    pub fn new(idx: i32 /*, content: String*/) -> Self {
        Object::builder()
            .property("completed", false)
            .property("idx", idx)
            .property("value", &"")
            .property("displayvalue", &"")
            .build()
    }
    pub fn new_from_cell(cell: api::Cell /*, content: String*/) -> Self {
        Object::builder()
            .property("completed", false)
            .property("idx", row_major_idx(cell.row, cell.col))
            .property("value", cell.value)
            .property("displayvalue", cell.display_value)
            .build()
    }
}

#[derive(Default)]
pub struct SpreadsheetCell {
    pub completed: bool,
    pub idx: i32,
    pub value: String,
    pub display_value: String,
}

fn row_major_idx(row: i32, col: i32) -> i32 {
    (row * NUM_COLS) + col
}
