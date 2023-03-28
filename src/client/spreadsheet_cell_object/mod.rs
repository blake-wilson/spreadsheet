mod imp;

use glib::object::ObjectExt;
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Entry};
use rpc_client::api;

const NUM_COLS: i32 = 10;
const NUM_ROWS: i32 = 20;

glib::wrapper! {
    pub struct SpreadsheetCellObject(ObjectSubclass<imp::SpreadsheetCellObject>) @extends gtk::Widget, gtk::Box;
}

impl SpreadsheetCellObject {
    pub fn new(idx: i32 /*, content: String*/) -> Self {
        println!("init new");
        let obj = Object::builder()
            .property("idx", idx)
            .property("value", &"")
            .property("displayvalue", &"")
            .build();
        println!("done setting up");
        obj
    }
    pub fn new_from_cell(cell: api::Cell) -> Self {
        Object::builder()
            .property("idx", row_major_idx(cell.row, cell.col))
            .property("value", cell.value)
            .property("displayvalue", cell.display_value)
            .build()
    }
}

impl SpreadsheetCellObject {
    pub fn bind(&self, formula_bar: &Entry) {
        // Get state
        let mut bindings = self.imp().bindings.borrow_mut();

        let entry = self.imp().entry.get();
        let value_binding = entry
            .bind_property("text", self, "value")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let display_value_binding = self
            .bind_property("displayvalue", &entry, "text")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        // let formula_bar_binding = entry
        //     .bind_property("text", formula_bar, "text")
        //     .flags(glib::BindingFlags::DEFAULT)
        //     .build();
        // Save binding
        bindings.push(value_binding);
        bindings.push(display_value_binding);
        // bindings.push(formula_bar_binding);
    }
    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
    pub fn entry_txt(&self) -> String {
        self.imp().entry.get().text().to_string()
    }
}

#[derive(Default)]
pub struct SpreadsheetCell {
    pub idx: i32,
    pub value: String,
    pub display_value: String,
}

fn row_major_idx(row: i32, col: i32) -> i32 {
    (row * NUM_COLS) + col
}
