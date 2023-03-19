use glib::signal::SignalHandlerId;
use glib::Binding;
use glib_macros::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Entry};
use std::cell::Cell;
use std::cell::RefCell;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/cell.ui")]
pub struct SpreadsheetCell {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SpreadsheetCell {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SpreadsheetCell";
    type Type = super::SpreadsheetCell;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SpreadsheetCell {
    fn constructed(&self) {
        self.parent_constructed();
        self.entry.connect_changed(move |entry| {
            println!("text is now {}", entry.text());
        });
    }
}

// Trait shared by all widgets
impl WidgetImpl for SpreadsheetCell {}

// Trait shared by all boxes
impl BoxImpl for SpreadsheetCell {}
