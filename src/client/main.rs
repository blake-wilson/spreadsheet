mod spreadsheet_cell_object;
mod ss_cell;

use gdk::Display;
use gdk4 as gdk;
use gio::traits::ListModelExt;
use gio::SimpleAction;
use glib::GString;
use glib_macros::clone;
use grpcio::ChannelBuilder;
use gtk::glib;
use gtk::prelude::BoxExt;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, Entry, EventControllerKey, Inhibit, ListItem,
    PropagationPhase, ScrolledWindow, SignalListItemFactory, SingleSelection,
};
use protobuf::RepeatedField;
use rpc_client::api::*;
use rpc_client::api_grpc::SpreadsheetApiClient;
use spreadsheet_cell_object::SpreadsheetCellObject;
use std::cell::Cell;
use std::cmp::{max, min};
use std::rc::Rc;
use std::sync::Arc;

const NUM_COLS: i32 = 10;
const NUM_ROWS: i32 = 20;

fn build_ui(application: &Application) {
    let grpc_env = Arc::new(grpcio::Environment::new(1));
    let api_client = Arc::new(SpreadsheetApiClient::new(
        ChannelBuilder::new(grpc_env).connect(&String::from("0.0.0.0:9090")),
    ));

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

    let action_formula_changed = SimpleAction::new_stateful(
        "formula-changed",
        Some(&String::static_variant_type()),
        String::from("").to_variant(),
    );
    action_formula_changed.connect_change_state(clone!(@weak formula_bar =>
        move |action, parameter| {
            let str_val: String = parameter
                .expect("Could not get parameter.")
                .get()
                .expect("needs to be a string");
            // formula_bar.set_text(str_val.as_str());
    }));

    let grid = build_grid(&formula_bar, api_client);
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
        .min_content_height(600)
        .child(&grid)
        .build();
    gtk_box.append(&scrolled_window);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .default_width(600)
        .default_height(650)
        .child(&gtk_box)
        .build();

    // Present the window
    // let key_controller = EventControllerKey::builder().build();
    window.present();
    // key_controller.connect_key_pressed(
    //     //clone!(@weak selection_model => @default-return Inhibit(false), move |_, key, key_code, _| {
    //     move |_, key, key_code, _| -> Inhibit {
    //         // selection_model.select_item(selection_model.selected() + 1, true);
    //         println!("key {} was clicked!", key_code);
    //         Inhibit(false)
    //         //}),
    //     },
    // );
    // key_controller.set_propagation_phase(PropagationPhase::Capture);
    // window.add_controller(key_controller);
    formula_bar.grab_focus();
}

// static VECTOR: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![
//     String::from("");
//     (NUM_COLS * NUM_ROWS) as usize
// ]));

fn build_grid(formula_bar: &gtk::Entry, api_client: Arc<SpreadsheetApiClient>) -> gtk::GridView {
    let mut req = GetCellsRequest::new();
    let mut get_rect = Rect::new();
    get_rect.set_start_row(0);
    get_rect.set_start_col(0);
    get_rect.set_stop_col(NUM_COLS);
    get_rect.set_stop_row(NUM_ROWS);
    req.set_rect(get_rect);
    let cells_resp = api_client.get_cells(&req).unwrap();

    let vector: Vec<SpreadsheetCellObject> = (0..(NUM_COLS * NUM_ROWS) as i32)
        .into_iter()
        .map(SpreadsheetCellObject::new)
        .collect();
    let cells: Vec<rpc_client::api::Cell> = cells_resp.cells.to_vec();
    cells
        .into_iter()
        .map(|c| {
            println!("setting cell object to {:#?}", c);
            let idx = row_major_idx(c.row, c.col) as usize;
            vector
                .get(idx)
                .unwrap()
                .set_property("displayvalue", c.display_value);
            vector.get(idx).unwrap().set_property("value", c.value);
        })
        .for_each(drop);
    // Create new model
    let model = gio::ListStore::new(SpreadsheetCellObject::static_type());
    // let model = ArcListModel::new();
    // Add the vector to the model
    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    // factory.connect_setup(move |_, list_item| {
    //let label = Label::builder()
    //    .max_width_chars(2)
    //    .build()
    // let entry = Entry::builder()
    //     .max_width_chars(8)
    //     .width_chars(8)
    //     .css_classes(vec![GString::from_string_unchecked(String::from(
    //         "ss_entry",
    //     ))])
    //     .build();
    // let cell = SpreadsheetCell::new();
    // let list_ref = list_item
    //     .downcast_ref::<ListItem>()
    //     .expect("Needs to be ListItem");
    // list_ref.set_activatable(false);
    // list_ref.set_child(Some(&cell));
    // });

    // let model_2 = model.downgrade();
    let selection_model = SingleSelection::new(Some(model));

    factory.connect_bind(clone!(@weak formula_bar, @weak selection_model => move |_, list_item| {
        let number;
         let lst_item = list_item.downcast_ref::<ListItem>().unwrap();
        if let Some(item) = lst_item.item() {
         let entry = Entry::builder()
             .max_width_chars(8)
             .width_chars(8)
             .css_classes(vec![GString::from_string_unchecked(String::from(
                 "ss_entry",
             ))])
             .build();
            number = item.property_value("idx").get::<i32>().unwrap();
            entry.set_text(item.property_value("displayvalue").get::<String>().unwrap().as_str());
            item.bind_property("value", &formula_bar, "text")
                .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::DEFAULT).build();
         entry.connect_changed(clone!(@weak formula_bar, @weak entry, @weak item =>
             move |_| {
                 // item.set_property("value", entry.text().as_str())
             }));
         let client = Arc::downgrade(&api_client);
         entry.connect_activate(clone!(@weak item, @weak entry, @weak selection_model =>
             move |_| {
                 println!("submitting formula {}", entry.text());
                 let mut req = InsertCellsRequest::new();
                 let cells = vec![new_insert_cell(number / NUM_COLS, number % NUM_COLS, &entry.text())];
                 req.set_cells(RepeatedField::from_vec(cells));
                 match client.upgrade() {
                     Some(c) => {
                         item.set_property("value", entry.text().as_str());
                         let resp = c.insert_cells(&req);
                         match resp {
                             Ok(cells) =>
                                 for cell in cells.cells {
                                     let model_idx = row_major_idx(cell.row, cell.col) as u32;
                                     let (num_removed, num_added) = (0 as u32, 0 as u32);
                                     let item = selection_model.item(model_idx);
                                     // item.unwrap().set_property("value", cell.value);
                                     println!("updating item {:#?} with value {:#?} at ({:?}, {:?})", item, cell.display_value, cell.row, cell.col);
                                     item.unwrap().set_property("displayvalue", cell.display_value);
                                     selection_model.emit_by_name::<()>("items-changed", &[&model_idx, &num_removed, &num_added]);
                                 }
                             Err(e) => println!("error inserting cells: {:?}", e)
                         }
                     }
                     None => {
                         println!("No API available!")
                     }
                 }
             }
         ));
         entry.connect_has_focus_notify(clone!(@weak formula_bar, @weak entry, @weak selection_model =>
             move |_| {
             println!("Notify\n\n");
             // entry.set_css_classes(&[&String::from("ss_entry_focused")]);
             // formula_bar.set_text(&entry.text());
             selection_model.select_item(number as u32, true);
             formula_bar.set_text(item.property_value("value").get::<String>().unwrap().as_str());
             entry.set_text(item.property_value("value").get::<String>().unwrap().as_str());
         }));

         lst_item.set_child(Some(&entry));
        }
     }));

    let grid = gtk::GridView::builder()
        .enable_rubberband(true)
        .factory(&factory)
        .model(&selection_model)
        .max_columns(NUM_COLS as u32)
        .min_columns(NUM_COLS as u32)
        .build();

    let key_controller = EventControllerKey::builder().build();
    key_controller.set_propagation_phase(PropagationPhase::Capture);
    key_controller.connect_key_pressed(
        clone!(@weak selection_model => @default-return Inhibit(false), move |_, _, key_code, _| {
            println!("current selection is {}", selection_model.selected());
            println!("current selected item is {:?}", selection_model.selected_item());
            let mut inhibit = true;
            // if key_code >= 123 && key_code <= 126 {
            //     selection_model.selected_item().unwrap().downcast_ref::<gtk::Widget>()
            //         .expect("Needs to be a Widget").grab_focus();
            // }
            if key_code == 123 { // left arrow
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 - 1) as u32, true);
            } else if key_code == 124 { // left arrow
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 + 1) as u32, true);
            } else if key_code == 125 { // down arrow
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 + NUM_COLS) as u32, true);
            } else if key_code == 126 { // up arrow
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 - NUM_COLS) as u32, true);
            } else {
                inhibit = false;
            }
            println!("key code is {}, inhibit is {}", key_code, inhibit);
            Inhibit(inhibit)
        }),
    );
    grid.add_controller(key_controller);

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

fn row_major_idx(row: i32, col: i32) -> i32 {
    (row * NUM_COLS) + col
}

fn clamp_selection(val: i32) -> i32 {
    min(max(val, 0), NUM_ROWS * NUM_COLS)
}

fn new_insert_cell(row: i32, col: i32, value: &str) -> InsertCell {
    let mut ic = InsertCell::new();
    ic.set_row(row);
    ic.set_col(col);
    ic.set_value(String::from(value));
    ic
}

fn main() {
    gio::resources_register_include!("templates.gresource").expect("Failed to register resources.");

    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    // Run the application
    app.run();
}
