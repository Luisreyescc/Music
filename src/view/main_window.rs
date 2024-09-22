use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, MessageDialog, ButtonsType};
use glib::clone;

pub fn create_main_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Music Manager v1")
        .default_width(400)
        .default_height(300)
        .build();

    let button = Button::with_label("Touch here to show Hello World");

    button.connect_clicked(clone!(@strong window => move |_| {
        let dialog = MessageDialog::builder()
            .transient_for(&window)
            .modal(true)
            .buttons(ButtonsType::Ok)
            .text("Hello World!")
            .build();
        
        dialog.connect_response(|dialog, response| {
            if response == gtk4::ResponseType::Ok {
                dialog.close(); 
            }
        });

        dialog.show();
    }));

    window.set_child(Some(&button));
    window.show();
}
