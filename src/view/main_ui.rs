use gtk::prelude::*; 
use gtk::{Button, Window, WindowType};

pub fn build_ui() {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("");
    window.set_default_size(800, 600);

    let button = Button::with_label("Click");

    window.add(&button);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}
