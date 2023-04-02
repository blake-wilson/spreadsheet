mod imp;

use crate::NUM_COLS;
use crate::NUM_ROWS;
use glib::object::ObjectExt;
use glib::Object;
use glib_macros::clone;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, EventControllerFocus, GestureClick, PropagationPhase, SingleSelection};
use rpc_client::api;

const ALPHABET: [char; 27] = [
    'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

glib::wrapper! {
    pub struct SpreadsheetCellObject(ObjectSubclass<imp::SpreadsheetCellObject>) @extends gtk::Widget, gtk::Box;
}

impl SpreadsheetCellObject {
    pub fn new(idx: i32 /*, content: String*/) -> Self {
        let obj: SpreadsheetCellObject = Object::builder()
            .property("idx", idx)
            .property("value", &"")
            .property("displayvalue", &"")
            .build();
        if idx == 0 {
        } else if idx % NUM_COLS == 0 {
            obj.set_property("displayvalue", (idx / NUM_COLS).to_string());
        } else if idx / NUM_COLS == 0 {
            obj.set_property("displayvalue", col_header_name(idx));
        }
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
        let mut bindings = self.imp().bindings.borrow_mut();

        let entry = self.imp().entry.get();
        let placeholder_binding = self
            .bind_property("displayvalue", &entry, "placeholder-text")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();

        let sensitive_binding = self
            .bind_property("idx", &entry, "sensitive")
            .transform_to(move |_, idx: i32| {
                Some((idx / NUM_COLS != 0 && (idx % NUM_COLS as i32) != 0).to_value())
            })
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        // Save binding
        let click_gesture = GestureClick::new();
        click_gesture.set_propagation_phase(PropagationPhase::Capture);
        let click_sig = click_gesture.connect_pressed(
            clone!(@weak self as this, @weak selection_model, @weak entry, @weak formula_bar => move |_, _, _, _| {
                let idx = this.property_value("idx").get::<i32>().unwrap();
                selection_model.select_item(idx as u32, true);
            }),
        );
        let focus_controller = EventControllerFocus::new();
        focus_controller.set_propagation_phase(PropagationPhase::Target);
        focus_controller.connect_leave(clone!(@weak self as this, @weak entry => move |_| {
            entry.set_text("");
        }));
        focus_controller.connect_enter(
            clone!(@weak self as this, @weak entry, @weak formula_bar => move |_| {
                let val = this.property_value("value").get::<String>().unwrap();
                entry.set_text(&val);
                formula_bar.set_text(&val);
            }),
        );
        clone!(@strong click_gesture => move || {
            self.add_controller(click_gesture);
        })();
        self.imp().gesture_handler.replace(Some(click_gesture));

        self.imp().click_signal.replace(Some(click_sig));

        clone!(@strong focus_controller => move || {
            self.add_controller(focus_controller);
        })();
        self.imp().focus_handler.replace(Some(focus_controller));
        bindings.push(placeholder_binding);
    }

    pub fn unbind(&self, clear_entry: bool) {
        if clear_entry {
            self.imp().entry.set_text("");
        }
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
        if let Some(ctrl) = self.imp().gesture_handler.take() {
            if let Some(sig_id) = self.imp().click_signal.take() {
                ctrl.disconnect(sig_id);
            }
            self.remove_controller(&ctrl);
        }
        if let Some(ctrl) = self.imp().focus_handler.take() {
            self.remove_controller(&ctrl);
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
        let value = self.property_value("value").get::<String>().unwrap();
        entry.connect_has_focus_notify(
            clone!(@weak entry, @weak selection_model, @weak formula_bar =>
                move |_| {
                    formula_bar.set_text(&value);
                    entry.set_text(&value);
            }),
        );
    }
    pub fn set_text(&self, txt: String) {
        self.imp().entry.set_text(&txt);
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

fn col_header_name(col_num: i32) -> String {
    let mut tmp = col_num;
    let mut new_val = String::from("");
    while tmp != 0 {
        new_val.push(ALPHABET[(tmp % 27) as usize]);
        tmp = tmp / 27;
    }
    new_val.chars().rev().collect::<String>()
}
