mod spreadsheet_cell_object;
mod ss_cell;

use gdk::Display;
use gdk4 as gdk;
use gio::traits::ListModelExt;
use glib_macros::clone;
use grpcio::ChannelBuilder;
use gtk::glib;
use gtk::glib::GString;
use gtk::prelude::BoxExt;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, EventControllerKey, Inhibit, ListItem, PropagationPhase,
    ScrolledWindow, SignalListItemFactory, SingleSelection,
};
use protobuf::RepeatedField;
use rpc_client::api::*;
use rpc_client::api_grpc::SpreadsheetApiClient;
use spreadsheet_cell_object::SpreadsheetCellObject;
use std::cmp::{max, min};
use std::sync::Arc;

const NUM_COLS: i32 = 36;
const NUM_ROWS: i32 = 72;

fn build_ui(application: &Application) {
    let grpc_env = Arc::new(grpcio::Environment::new(1));
    let api_client = Arc::new(SpreadsheetApiClient::new(
        ChannelBuilder::new(grpc_env).connect(&String::from("0.0.0.0:9090")),
    ));

    let formula_bar = gtk::Entry::builder()
        // .width_chars(100)
        // .width_request(200)
        .max_width_chars(100)
        .css_classes(vec![GString::from_string_unchecked(String::from(
            "formula_bar",
        ))])
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
        .default_width(800)
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

    factory.connect_setup(
        clone!(@weak formula_bar, @weak selection_model => move |_, list_item| {
            let lst_item = list_item
                .downcast_ref::<ListItem>()
                .expect("needs to be a ListItem");
            let ss_cell = SpreadsheetCellObject::new(0);
            match lst_item.item() {
                Some(item) => {
                    let idx = item.property_value("idx")
                        .get::<i32>()
                        .expect("index needs to be a i32");
                    ss_cell.set_property("idx", idx);
                    let dv = item.property_value("displayvalue")
                        .get::<String>()
                        .expect("displayvalue needs to be a String");
                    ss_cell.set_property("displayvalue", dv.as_str());
                },
                None => (),
            }
            lst_item.set_child(Some(&ss_cell));
        }),
    );
    selection_model.connect_selection_changed(clone!(@weak selection_model => move |_, _, _| {
        let widget = selection_model.selected_item()
            .unwrap();
        let cell = widget
                .downcast_ref::<SpreadsheetCellObject>()
                .expect("The widget must be a `SpreadsheetCellObject`.");
        cell.focus();
    }));

    factory.connect_bind(clone!(@weak formula_bar, @weak selection_model => move |_, list_item| {
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

    factory.connect_unbind(clone!(@weak selection_model => move |_, list_item| {
        let cell = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<SpreadsheetCellObject>()
            .expect("The child has to be an `SpreadsheetCellObject`.");
        let cell_idx = cell.property_value("idx").get::<i32>().unwrap();
        let clear_entry = selection_model.selected() != cell_idx as u32;
        cell.unbind(clear_entry);
    }));

    let grid = gtk::GridView::builder()
        .enable_rubberband(true)
        .halign(gtk::Align::Fill)
        .factory(&factory)
        .model(&selection_model)
        .max_columns(NUM_COLS as u32)
        .min_columns(NUM_COLS as u32)
        .width_request(200)
        .build();

    let key_controller = EventControllerKey::builder().build();
    key_controller.set_propagation_phase(PropagationPhase::Capture);
    key_controller.connect_key_pressed(
        clone!(@weak selection_model => @default-return Inhibit(false), move |_, _, key_code, _| {
            let mut inhibit = true;
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
                let idx = sel.property_value("idx").get::<i32>().unwrap();
                let item = selection_model.item(idx as u32).expect("item needs to be a GObject");
                let cell = sel.downcast::<SpreadsheetCellObject>().unwrap();
                let cell_val = cell.entry_txt();
                item.set_property("value", cell_val.clone());
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
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 + NUM_COLS) as u32, true);
            } else {
                inhibit = false;
            }
            println!("key code is {}, inhibit is {}", key_code, inhibit);
            Inhibit(inhibit)
        }),
    );
    grid.add_controller(key_controller);

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
