mod imp;

use glib::object::ObjectExt;
use glib::ParamSpec;
use glib::{Object, SignalHandlerId};
use glib_macros::clone;
use gtk::prelude::*;
use gtk::prelude::{EntryExt, WidgetExt};
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, EventControllerFocus, GestureClick, PropagationPhase, SingleSelection};
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
        let placeholder_binding = self
            .bind_property("displayvalue", &entry, "placeholder-text")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        // Save binding
        let click_gesture = GestureClick::new();
        click_gesture.set_propagation_phase(PropagationPhase::Capture);
        let id = click_gesture.connect_pressed(
            clone!(@weak self as this, @weak selection_model, @weak entry, @weak formula_bar => move |_, _, _, _| {
                let idx = this.property_value("idx").get::<i32>().unwrap();
                println!("selecting idx {}", idx);
                selection_model.select_item(idx as u32, true);
            }),
        );
        let focus_controller = EventControllerFocus::new();
        let focus_handler_id =
            focus_controller.connect_leave(clone!(@weak self as this, @weak entry => move |_| {
                entry.set_text("");
            }));
        self.imp()
            .gesture_handler
            .replace(Some(click_gesture.clone()));
        self.imp()
            .focus_handler
            .replace(Some(focus_controller.clone()));
        entry.add_controller(focus_controller);
        self.add_controller(click_gesture);
        bindings.push(placeholder_binding);
    }
    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
        if let Some(ctrl) = self.imp().gesture_handler.take() {
            self.remove_controller(&ctrl);
        }
        if let Some(ctrl) = self.imp().focus_handler.take() {
            self.imp().entry.remove_controller(&ctrl);
        }
    }
    pub fn entry_txt(&self) -> String {
        self.imp().entry.get().text().to_string()
    }
    pub fn focus(&self) {
        self.imp().entry.grab_focus();
    }
    pub fn connect(&self, selection_model: &SingleSelection, formula_bar: &Entry) {
        let entry = self.imp().entry.get();
        let idx = self.property_value("idx").get::<i32>().unwrap();
        let value = self.property_value("value").get::<String>().unwrap();
        entry.connect_has_focus_notify(
            clone!(@weak entry, @weak selection_model, @weak formula_bar =>
                move |_| {
                    formula_bar.set_text(&value);
                    entry.set_text(&value);
            }),
        );
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
