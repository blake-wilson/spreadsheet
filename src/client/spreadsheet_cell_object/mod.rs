mod imp;

use glib::object::ObjectExt;
use glib::{Object, SignalHandlerId};
use glib_macros::clone;
use gtk::prelude::*;
use gtk::prelude::{EntryExt, WidgetExt};
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, GestureClick, PropagationPhase, SingleSelection};
use rpc_client::api;
use std::cell::Ref;
use std::cell::RefCell;

const NUM_COLS: i32 = 10;
const NUM_ROWS: i32 = 20;

glib::wrapper! {
    pub struct SpreadsheetCellObject(ObjectSubclass<imp::SpreadsheetCellObject>) @extends gtk::Widget, gtk::Box;
}

impl SpreadsheetCellObject {
    pub fn new(idx: i32 /*, content: String*/) -> Self {
        let obj = Object::builder()
            .property("idx", idx)
            .property("value", &"")
            .property("displayvalue", &"")
            .build();
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
    pub fn bind(&self, selection_model: &SingleSelection, formula_bar: &Entry) {
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
        let click_gesture = GestureClick::new();
        click_gesture.set_propagation_phase(PropagationPhase::Capture);
        let id = click_gesture.connect_pressed(
            clone!(@weak self as this, @weak selection_model => move |_, _, _, _| {
                this.focus();
                let idx = this.property_value("idx").get::<i32>().unwrap();
                println!("selecting idx {}", idx);
                selection_model.select_item(idx as u32, true);
            }),
        );
        self.imp().gesture_handler.replace(Some(id));

        self.add_controller(click_gesture);
        bindings.push(value_binding);
        bindings.push(display_value_binding);
        // bindings.push(formula_bar_binding);
    }
    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
        if let Some(id) = self.imp().gesture_handler.take() {
            self.disconnect(id);
        }
    }
    pub fn entry_txt(&self) -> String {
        self.imp().entry.get().text().to_string()
    }
    pub fn focus(&self) {
        println!(
            "focusing cell {:?}",
            self.property_value("idx").get::<i32>().unwrap()
        );
        self.imp().entry.grab_focus();
    }
    pub fn connect(&self, selection_model: &SingleSelection) {
        let entry = self.imp().entry.get();
        let idx = self.property_value("idx").get::<i32>().unwrap();
        entry.connect_has_focus_notify(clone!(@weak entry, @weak selection_model =>
            move |_| {
                println!("selecting item {}", idx);
                selection_model.select_item(idx as u32, true);
        }));
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
