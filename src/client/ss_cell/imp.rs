use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use std::cell::RefCell;

// Object holding the state
#[derive(Default, Debug, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/cell.ui")]
pub struct SpreadsheetCell {
    #[template_child]
    pub entry: TemplateChild<gtk::Entry>,
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SpreadsheetCell {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SpreadsheetCell";
    type Type = super::SpreadsheetCell;
    type ParentType = gtk::Entry;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SpreadsheetCell {
    fn constructed(&self) {
        self.parent_constructed();
        // self.connect_changed(move |entry| {
        //     println!("text is now {}", entry.text());
        // });
    }
}

// Trait shared by all widgets
impl WidgetImpl for SpreadsheetCell {}

impl EntryImpl for SpreadsheetCell {}
