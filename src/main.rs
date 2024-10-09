mod model;
mod controller;
mod view;

use model::database_config::{config, database_tables, populate_db};

fn main() {
    initialize_ui();
}

fn initialize_ui() {
    gtk::init().expect("Failed to initialize GTK.");
    view::main_ui::build_ui();
    gtk::main();
}
