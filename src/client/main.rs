mod ss_cell;

use gdk::Display;
use gdk4 as gdk;
use glib::GString;
use glib::Object;
use glib::StrV;
use glib_macros::clone;
use gtk::glib;
use gtk::prelude::BoxExt;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, Entry, ListItem, ScrolledWindow, SignalListItemFactory,
    SingleSelection,
};
use ss_cell::IntegerObject;
use std::cell::Cell;
use std::rc::Rc;

// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // … create a new window …
    let window = gtk::ApplicationWindow::new(application);
    // … with a button in it …
    let button = gtk::Button::with_label("Hello World!");
    // … which closes the window when clicked
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&button));
    window.present();
}

fn build_ui(application: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(
        clone!(@weak number, @weak button_decrease, @weak button_increase =>
            move |_| {
                number.set(number.get() + 1);
                button_decrease.set_label(&number.get().to_string());
                button_increase.set_label(&number.get().to_string());
        }),
    );
    button_decrease.connect_clicked(clone!(@weak button_increase, @weak button_decrease =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
            button_decrease.set_label(&number.get().to_string());
    }));

    // Add buttons to `gtk_box`

    let formula_bar = gtk::Entry::builder()
        .width_chars(100)
        .max_width_chars(100)
        .build();
    let grid = build_grid(&formula_bar);
    grid.set_size_request(800, 600);

    // let layout_grid = gtk::Grid::builder()
    //     .row_homogeneous(false)
    //     .column_homogeneous(false)
    //     .build();
    // layout_grid.attach(&formula_bar, 0, 0, 1, 1);
    // layout_grid.attach(&grid, 0, 1, 1, 1);

    let gtk_box = gtk::Box::builder()
        .margin_top(2)
        .margin_bottom(2)
        .margin_start(2)
        .margin_end(2)
        .orientation(gtk::Orientation::Vertical)
        .homogeneous(false)
        .build();
    gtk_box.append(&formula_bar);

    let scrolled_window = ScrolledWindow::builder()
        //.hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .min_content_height(300)
        .child(&grid)
        .build();
    gtk_box.append(&scrolled_window);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .default_width(1200)
        .default_height(800)
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}

fn build_grid(formula_bar: &gtk::Entry) -> gtk::GridView {
    let vector: Vec<IntegerObject> = (0..=100).into_iter().map(IntegerObject::new).collect();
    // Create new model
    let model = gio::ListStore::new(IntegerObject::static_type());
    // Add the vector to the model
    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        //let label = Label::builder()
        //    .max_width_chars(2)
        //    .build()
        let entry = Entry::builder()
            .max_width_chars(8)
            .width_chars(8)
            .css_classes(vec![GString::from_string_unchecked(String::from(
                "ss_entry",
            ))])
            .build();
        let list_ref = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        list_ref.set_selectable(false);
        list_ref.set_child(Some(&entry));
    });

    factory.connect_bind(clone!(@weak formula_bar => move |_, list_item| {
        // Get `IntegerObject` from `ListItem`
        let integer_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Get `i32` from `IntegerObject`
        let number = integer_object.property::<i32>("number");

        // Get `Label` from `ListItem`
        let entry = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<Entry>()
            .expect("The child has to be a `Entry`.");
        entry.connect_changed(clone!(@weak formula_bar, @weak entry =>
            move |_| {
                formula_bar.set_text(&entry.text());
        }));
        entry.connect_has_focus_notify(clone!(@weak formula_bar, @weak entry =>
            move |_| {
            // entry.set_css_classes(&[&String::from("ss_entry_focused")]);
            formula_bar.set_text(&entry.text());
        }));

        entry.set_text(&number.to_string());
    }));

    let selection_model = SingleSelection::new(Some(model));
    let grid = gtk::GridView::builder()
        .enable_rubberband(true)
        .factory(&factory)
        .model(&selection_model)
        .max_columns(20)
        .min_columns(20)
        .build();

    // let cell_width = 1;
    // let cell_height = 1;
    // // rows
    // for i in 1..10 {
    //     //cols
    //     for j in 1..8 {
    //         let txt_edit = gtk::Entry::builder()
    //             .margin_top(2)
    //             .margin_bottom(2)
    //             .margin_start(2)
    //             .margin_end(2)
    //             .width_chars(8)
    //             .max_width_chars(8)
    //             .css_classes(vec![GString::from_string_unchecked(String::from(
    //                 "ss_entry",
    //             ))])
    //             .build();
    //         txt_edit.connect_changed(clone!(@weak formula_bar, @weak txt_edit =>
    //             move |_| {
    //                 formula_bar.set_text(&txt_edit.text());
    //         }));
    //         txt_edit.connect_has_focus_notify(clone!(@weak formula_bar, @weak txt_edit =>
    //             move |_| {
    //             txt_edit.set_css_classes(&[&String::from("ss_entry_focused")]);
    //                 formula_bar.set_text(&txt_edit.text());
    //         }));
    //         // grid.attach(&txt_edit, j * cell_width, i * cell_height, 1, 1);
    //     }
    // }
    grid
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    // Run the application
    app.run();
}
