mod imp;

use crate::spreadsheet_cell_object::SpreadsheetCellObject;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango, Entry};
use pango::{AttrInt, AttrList};

const NUM_COLS: i32 = 20;
const NUM_ROWS: i32 = 10;

glib::wrapper! {
    pub struct SpreadsheetCell(ObjectSubclass<imp::SpreadsheetCell>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SpreadsheetCell {
    pub fn new() -> Self {
        Object::builder().build()
    }

    // ANCHOR: bind
    pub fn bind(&self, ss_cell: &SpreadsheetCellObject) {
        // Get state
        let entry = self.imp().entry.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `ss_cell.completed` to `task_row.completed_button.active`
        let entry_binding = ss_cell
            .bind_property("completed", &entry, "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        // Save binding
        bindings.push(entry_binding);
    }
    // ANCHOR_END: bind

    // ANCHOR: unbind
    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    pub fn entry(&self) -> Entry {
        self.imp().entry.get()
    }

    // ANCHOR_END: unbind
}
