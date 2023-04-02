use spreadsheet_service::{models, service};
mod spreadsheet_cell_object;
mod ss_cell;

use gdk::Display;
use gdk4 as gdk;
use gio::traits::ListModelExt;
use glib_macros::clone;
use gtk::glib;
use gtk::glib::GString;
use gtk::prelude::BoxExt;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, EventControllerKey, Inhibit, ListItem, PropagationPhase,
    ScrolledWindow, SignalListItemFactory, SingleSelection,
};
use spreadsheet_cell_object::SpreadsheetCellObject;
use std::cmp::{max, min};
use std::sync::{Arc, RwLock};

const NUM_COLS: i32 = 36;
const NUM_ROWS: i32 = 150;

fn build_ui(application: &Application) {
    // let grpc_env = Arc::new(grpcio::Environment::new(1));
    // let api_client = Arc::new(SpreadsheetApiClient::new(
    //     ChannelBuilder::new(grpc_env).connect(&String::from("0.0.0.0:9090")),
    // ));
    let ss_service = Arc::new(RwLock::new(service::MemoryCellsService::new(
        NUM_ROWS, NUM_COLS,
    )));

    let formula_bar = gtk::Entry::builder()
        // .width_chars(100)
        // .width_request(200)
        .max_width_chars(100)
        .css_classes(vec![GString::from_string_unchecked(String::from(
            "formula_bar",
        ))])
        .build();

    let grid = build_grid(&formula_bar, Arc::clone(&ss_service));
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
        // .min_content_height(600)
        .child(&grid)
        .build();
    gtk_box.append(&scrolled_window);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Spreadsheet")
        .default_width(800)
        .default_height(650)
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
    formula_bar.grab_focus();
}

fn build_grid<T: service::CellsService + 'static>(
    formula_bar: &gtk::Entry,
    service: Arc<RwLock<T>>,
) -> gtk::GridView {
    let cells = get_all_cells(Arc::clone(&service));

    let vector: Vec<SpreadsheetCellObject> = (0..(NUM_COLS * NUM_ROWS) as i32)
        .into_iter()
        .map(SpreadsheetCellObject::new)
        .collect();
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
    selection_model.connect_selection_changed(
        clone!(@weak selection_model, @weak formula_bar => move |_, _, _| {
            let widget = selection_model.selected_item()
                .unwrap();
            let cell = widget
                    .downcast_ref::<SpreadsheetCellObject>()
                    .expect("The widget must be a `SpreadsheetCellObject`.");
            cell.focus();
        }),
    );
    formula_bar.connect_changed(
        clone!(@weak selection_model, @weak formula_bar => move |_| {
            let widget = selection_model.selected_item()
                .unwrap();
            let cell = widget
                    .downcast_ref::<SpreadsheetCellObject>()
                    .expect("The widget must be a `SpreadsheetCellObject`.");
            cell.set_text(formula_bar.text().to_string());
        }),
    );
    formula_bar.connect_activate(
        clone!(@weak formula_bar, @weak selection_model, @weak service => move |_| {
            let sel = selection_model.selected_item().unwrap();
            let idx = sel.property_value("idx").get::<i32>().unwrap();
            let item = selection_model.item(idx as u32).expect("item needs to be a GObject");
            let cell = sel.downcast::<SpreadsheetCellObject>().unwrap();
            let cell_val = formula_bar.text();
            item.set_property("value", cell_val.clone());
            insert_cell(&cell, &selection_model, Arc::clone(&service));

            selection_model.select_item(clamp_selection(selection_model.selected() as i32 + NUM_COLS) as u32, true);
        }),
    );

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
        .hexpand(true)
        .vexpand(true)
        .build();

    let key_controller = EventControllerKey::builder().build();
    key_controller.set_propagation_phase(PropagationPhase::Capture);
    key_controller.connect_key_pressed(
        clone!(@weak selection_model => @default-return Inhibit(false), move |_, key_val, _, _| {
            if key_val.name().is_none() {
                return Inhibit(false);
            }
            let name = key_val.name().unwrap();
            let mut inhibit = true;
            if name == "Left" {
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 - 1) as u32, true);
            } else if name == "Right" || name == "Tab"{
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 + 1) as u32, true);
            } else if name == "Down" {
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 + NUM_COLS) as u32, true);
            } else if name == "Up" {
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 - NUM_COLS) as u32, true);
            } else if name == "Return" { // ENTER
                let sel = selection_model.selected_item().unwrap();
                let idx = sel.property_value("idx").get::<i32>().unwrap();
                let item = selection_model.item(idx as u32).expect("item needs to be a GObject");
                let cell = sel.downcast::<SpreadsheetCellObject>().unwrap();
                let cell_val = cell.entry_txt();
                item.set_property("value", cell_val.clone());
                insert_cell(&cell, &selection_model, Arc::clone(&service));
                selection_model.select_item(clamp_selection(selection_model.selected() as i32 + NUM_COLS) as u32, true);
            } else {
                inhibit = false;
            }
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

fn get_all_cells<T: service::CellsService>(service: Arc<RwLock<T>>) -> Vec<models::Cell> {
    let get_rect = models::Rect {
        start_row: 0,
        stop_row: NUM_ROWS,
        start_col: 0,
        stop_col: NUM_COLS,
    };

    service.write().unwrap().get_cells(get_rect)
}

fn insert_cell<T: service::CellsService>(
    cell: &SpreadsheetCellObject,
    selection_model: &SingleSelection,
    service: Arc<RwLock<T>>,
) {
    let idx = cell.property_value("idx").get::<i32>().unwrap();
    let cell_val = cell.property_value("value").get::<String>().unwrap();
    if cell_val == "" {
        return;
    }

    let cells = vec![models::Cell::new(idx / NUM_COLS, idx % NUM_COLS, cell_val)];
    let resp = service.write().unwrap().insert_cells(&cells);
    match resp {
        Ok(cells) => {
            for cell in cells {
                let model_idx = row_major_idx(cell.row, cell.col) as u32;
                let (num_removed, num_added) = (0 as u32, 0 as u32);
                let item = selection_model
                    .item(model_idx)
                    .expect("item needs to be a GObject");
                item.set_property("displayvalue", cell.display_value);
                selection_model
                    .emit_by_name::<()>("items-changed", &[&model_idx, &num_removed, &num_added]);
            }
        }
        Err(e) => println!("error inserting cells: {:?}", e),
    }
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
