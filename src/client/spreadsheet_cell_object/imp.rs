use std::cell::RefCell;
use std::rc::Rc;

use glib::{ParamSpec, ParamSpecBoolean, ParamSpecInt, ParamSpecString, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use super::SpreadsheetCell;

// ANCHOR: struct_and_subclass
// Object holding the state
#[derive(Default)]
pub struct SpreadsheetCellObject {
    pub data: Rc<RefCell<SpreadsheetCell>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SpreadsheetCellObject {
    const NAME: &'static str = "SpreadsheetCellObject";
    type Type = super::SpreadsheetCellObject;
}
// ANCHOR_END: struct_and_subclass

// Trait shared by all GObjects
impl ObjectImpl for SpreadsheetCellObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecBoolean::builder("completed").build(),
                ParamSpecInt::builder("idx").build(),
                ParamSpecString::builder("value").build(),
                ParamSpecString::builder("displayvalue").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "completed" => {
                let input_value = value.get().expect("The value needs to be of type `bool`.");
                self.data.borrow_mut().completed = input_value;
            }
            "idx" => {
                let input_value = value.get().expect("The value needs to be of type `int`.");
                self.data.borrow_mut().idx = input_value;
            }
            "value" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().value = input_value;
            }
            "displayvalue" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().display_value = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "completed" => self.data.borrow().completed.to_value(),
            "idx" => self.data.borrow().idx.to_value(),
            "value" => self.data.borrow().value.to_value(),
            "displayvalue" => self.data.borrow().display_value.to_value(),
            _ => unimplemented!(),
        }
    }
}
