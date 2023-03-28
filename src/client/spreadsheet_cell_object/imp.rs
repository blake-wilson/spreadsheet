use std::cell::RefCell;
use std::rc::Rc;

use glib::subclass::InitializingObject;
use glib::Binding;
use glib::{ParamSpec, ParamSpecBoolean, ParamSpecInt, ParamSpecString, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Entry};
use once_cell::sync::Lazy;

use super::SpreadsheetCell;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/cell.ui")]
pub struct SpreadsheetCellObject {
    #[template_child]
    pub entry: TemplateChild<Entry>,

    pub data: Rc<RefCell<SpreadsheetCell>>,
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SpreadsheetCellObject {
    const NAME: &'static str = "SpreadsheetCellObject";
    type Type = super::SpreadsheetCellObject;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SpreadsheetCellObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecInt::builder("idx").build(),
                ParamSpecString::builder("value").build(),
                ParamSpecString::builder("displayvalue").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        println!("setting property {} {:#?}", pspec.name(), value);
        match pspec.name() {
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
        println!("done setting property");
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        println!("getting property {:#?}", pspec.name());
        let ret = match pspec.name() {
            "idx" => self.data.borrow().idx.to_value(),
            "value" => self.data.borrow().value.to_value(),
            "displayvalue" => self.data.borrow().display_value.to_value(),
            _ => unimplemented!(),
        };
        println!("got property value {:#?}", ret);
        ret
    }
}

impl WidgetImpl for SpreadsheetCellObject {}

impl EntryImpl for SpreadsheetCellObject {}

impl BoxImpl for SpreadsheetCellObject {}
