use glib::Object;
use gtk::glib;
mod imp;

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
            .build()
    }
}

#[derive(Default)]
pub struct SpreadsheetCell {
    pub completed: bool,
    pub idx: i32,
    pub value: String,
}
