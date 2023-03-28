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

    let formula_bar = gtk::Entry::builder()
        .width_chars(100)
        .max_width_chars(100)
        .build();

    let grid = build_grid(&formula_bar, api_client);
    grid.set_size_request(800, 600);

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
        .min_content_width(340)
        .min_content_height(600)
        .child(&grid)
        .build();
    gtk_box.append(&scrolled_window);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .default_width(400)
        .default_height(650)
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
    formula_bar.grab_focus();
}

fn build_grid(formula_bar: &gtk::Entry, api_client: Arc<SpreadsheetApiClient>) -> gtk::GridView {
    let mut req = GetCellsRequest::new();
    let mut get_rect = Rect::new();
    get_rect.set_start_row(0);
    get_rect.set_start_col(0);
    get_rect.set_stop_col(NUM_COLS);
    get_rect.set_stop_row(NUM_ROWS);
    req.set_rect(get_rect);
    let cells_resp = api_client
        .get_cells(&req)
        .expect("failed to get initial API response");

    let vector: Vec<SpreadsheetCellObject> = (0..(NUM_COLS * NUM_ROWS) as i32)
        .into_iter()
        .map(SpreadsheetCellObject::new)
        .collect();
    let cells: Vec<rpc_client::api::Cell> = cells_resp.cells.to_vec();
    cells
        .into_iter()
        .map(|c| {
            let idx = row_major_idx(c.row, c.col) as usize;
            vector
                .get(idx)
                .expect("needs to be a SpreadsheetCellObject")
                .set_property("displayvalue", c.display_value);
            vector
                .get(idx)
                .expect(&format!("must have entry for idx {:?}", idx))
                .set_property("value", c.value);
        })
        .for_each(drop);
    // Create new model
    let model = gio::ListStore::new(SpreadsheetCellObject::static_type());
    // let model = ArcListModel::new();
    // Add the vector to the model
    model.extend_from_slice(&vector);
    let selection_model = SingleSelection::new(Some(model));
    let factory = SignalListItemFactory::new();

    factory.connect_setup(clone!(@weak formula_bar => move |_, list_item| {
        let lst_item = list_item
            .downcast_ref::<ListItem>()
            .expect("needs to be a ListItem");
        let ss_cell = SpreadsheetCellObject::new(0);
        match lst_item.item() {
            Some(item) => {
                let dv = item.property_value("displayvalue")
                    .get::<String>()
                    .expect("displayvalue needs to be a String");
                ss_cell.set_property("displayvalue", dv.as_str());
            },
            None => (),
        }
        lst_item.set_child(Some(&ss_cell));
    }));

    factory.connect_bind(clone!(@weak formula_bar, @weak selection_model => move |_, list_item| {
        println!("binding item");
        let cell = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<SpreadsheetCellObject>()
                .expect("The child has to be an `SpreadsheetCellObject`.");

        let lst_item = list_item.downcast_ref::<ListItem>().expect("needs to be a ListItem");
        if let Some(item) = lst_item.item() {
            cell.set_property("displayvalue", item.property_value("displayvalue").get::<String>().expect("displayvalue needs to be a String").as_str());
        }
        cell.bind(&selection_model, &formula_bar);
        lst_item.set_child(Some(&cell));
     }));

    let grid = gtk::GridView::builder()
        .enable_rubberband(true)
        .halign(gtk::Align::Fill)
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
            } else if key_code == 36 { // ENTER
                let sel = selection_model.selected_item().unwrap();
                println!("selection is {:#?}", sel);
                let idx = sel.property_value("idx").get::<i32>().unwrap();
                let item = selection_model.item(idx as u32).expect("item needs to be a GObject");
                let cell_val = item.property_value("value").get::<String>().unwrap();
                let entry = sel.downcast::<SpreadsheetCellObject>().unwrap();
                println!("entry text: {}", entry.entry_txt());
                let client = Arc::downgrade(&api_client);
                println!("submitting formula at {}: {:#?}", idx, cell_val);
                let mut req = InsertCellsRequest::new();
                let cells = vec![new_insert_cell(idx / NUM_COLS, idx % NUM_COLS, &cell_val)];
                req.set_cells(RepeatedField::from_vec(cells));
                match client.upgrade() {
                    Some(c) => {
                        let resp = c.insert_cells(&req);
                        match resp {
                            Ok(cells) =>
                                for cell in cells.cells {
                                    println!("updating cell {:#?}", cell);
                                    let model_idx = row_major_idx(cell.row, cell.col) as u32;
                                    let (num_removed, num_added) = (0 as u32, 0 as u32);
                                    let item = selection_model.item(model_idx).expect("item needs to be a GObject");
                                    println!("updating item {:#?} with value {:#?} at ({:?}, {:?})", item, cell.display_value, cell.row, cell.col);
                                    item.set_property("displayvalue", cell.display_value);
                                    selection_model.emit_by_name::<()>("items-changed", &[&model_idx, &num_removed, &num_added]);
                                }
                            Err(e) => println!("error inserting cells: {:?}", e)
                        }
                    }
                    None => {
                        println!("No API available!")
                    }
                }
            } else {
                // let sel = selection_model.selected_item().unwrap();
                // let idx = sel.property_value("idx").get::<i32>().unwrap();
                // let (num_removed, num_added) = (0 as u32, 0 as u32);
                // selection_model.emit_by_name::<()>("items-changed", &[&(idx as u32), &num_removed, &num_added]);
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
