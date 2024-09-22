mod view {
    pub mod main_window; 
}

use gtk4::prelude::*;
use gtk4::Application;

fn main() {
    let app = Application::builder()
        .application_id("com.example.MusicManager")
        .build();

    app.connect_activate(|app| {
        view::main_window::create_main_window(app);
    });

    app.run();
}
