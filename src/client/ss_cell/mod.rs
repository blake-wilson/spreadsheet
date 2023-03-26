mod imp;

use crate::spreadsheet_cell_object::SpreadsheetCellObject;
use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct SpreadsheetCell(ObjectSubclass<imp::SpreadsheetCell>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SpreadsheetCell {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, _: &SpreadsheetCellObject) {}

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

#[gtk::template_callbacks]
impl SpreadsheetCell {
    #[template_callback]
    fn entry_changed(&self, _entry: &gtk::Entry) {
        // _entry.set_text("I was clicked!");
        println!("callback: entry is now this");
    }
}
