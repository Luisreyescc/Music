use gtk::prelude::*;
use gtk::{ProgressBar, Button, TreeView, TreeViewColumn, CellRendererText, Box as GtkBox, Orientation, Window, WindowType, Label, Entry, ScrolledWindow, ListStore, Settings, MenuButton, Popover};
use crate::controller::controller::{populate_song_list, save_directory_to_config, run_data_pipeline, show_error_dialog};
use gtk::traits::SettingsExt;
use std::rc::Rc;
use std::cell::RefCell;

pub fn build_ui() {
    gtk::init().expect("Failed to initialize GTK.");

    let settings = Settings::default().expect("Failed to get default settings");

    let dark_mode_enabled = Rc::new(RefCell::new(true));
    settings.set_gtk_application_prefer_dark_theme(*dark_mode_enabled.borrow());

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Music Manager");
    window.set_default_size(800, 600);

    let main_box = GtkBox::new(Orientation::Vertical, 5);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let header_box = GtkBox::new(Orientation::Horizontal, 0);
    header_box.set_halign(gtk::Align::End);
    header_box.set_valign(gtk::Align::Start);

    let menu_button = MenuButton::new();
    menu_button.set_label("☰");

    let popover = Popover::new(Some(&menu_button));
    let popover_box = GtkBox::new(Orientation::Vertical, 5);
    let toggle_theme_button = Button::with_label("Toggle Dark Mode");

    popover_box.pack_start(&toggle_theme_button, false, false, 5);
    popover.add(&popover_box);
    popover_box.show_all();

    menu_button.set_popover(Some(&popover));
    header_box.pack_start(&menu_button, false, false, 5);

    main_box.pack_start(&header_box, false, false, 0);

    let content_box = GtkBox::new(Orientation::Horizontal, 5);

    let song_list_box = GtkBox::new(Orientation::Vertical, 5);
    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    let tree_view = TreeView::new();
    let refresh_button = Button::with_label("Refresh");
    let progress_bar = ProgressBar::new();

    let list_store = ListStore::new(&[glib::Type::STRING, glib::Type::STRING, glib::Type::STRING]);

    tree_view.set_model(Some(&list_store));

    let renderer = CellRendererText::new();
    
    let col_title = TreeViewColumn::new();
    col_title.set_title("Title");
    col_title.pack_start(&renderer, true);
    col_title.add_attribute(&renderer, "text", 0);

    let col_artist = TreeViewColumn::new();
    col_artist.set_title("Artist(s)");
    col_artist.pack_start(&renderer, true);
    col_artist.add_attribute(&renderer, "text", 1);

    let col_album = TreeViewColumn::new();
    col_album.set_title("Album");
    col_album.pack_start(&renderer, true);
    col_album.add_attribute(&renderer, "text", 2);

    tree_view.append_column(&col_title);
    tree_view.append_column(&col_artist);
    tree_view.append_column(&col_album);

    scrolled_window.add(&tree_view);
    song_list_box.pack_start(&scrolled_window, true, true, 0);

    let right_box = GtkBox::new(Orientation::Vertical, 5);
    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("Search ..."));
    right_box.pack_start(&search_entry, false, false, 0);
    
    let directory_entry = Entry::new();
    directory_entry.set_placeholder_text(Some("Enter music directory here..."));
    song_list_box.pack_start(&directory_entry, false, false, 5);
    song_list_box.pack_start(&refresh_button, false, false, 5);

    let details_box = GtkBox::new(Orientation::Vertical, 5);
    
    // Right box info example
    let label_title = Label::new(Some("Song Metrics"));
    let label_performer = Label::new(Some("Performer(s): Coldplay"));
    let label_song = Label::new(Some("Title: Yellow"));
    let label_album = Label::new(Some("Album: Parachutes"));
    let label_year = Label::new(Some("Year: 2000"));
    let label_genre = Label::new(Some("Content type: Alternative rock"));
    let label_track = Label::new(Some("Track number (in album): 5"));

    details_box.pack_start(&label_title, false, false, 5);
    details_box.pack_start(&label_performer, false, false, 5);
    details_box.pack_start(&label_song, false, false, 5);
    details_box.pack_start(&label_album, false, false, 5);
    details_box.pack_start(&label_year, false, false, 5);
    details_box.pack_start(&label_genre, false, false, 5);
    details_box.pack_start(&label_track, false, false, 5);

    right_box.set_size_request(500, -1);
    right_box.pack_start(&details_box, true, true, 0);

    content_box.pack_start(&song_list_box, true, true, 0);
    content_box.pack_start(&right_box, false, false, 0);
    right_box.pack_start(&progress_bar, false, false, 5);

    main_box.pack_start(&content_box, true, true, 0);

    window.add(&main_box);

    window.show_all();

    let window_clone = window.clone();
    refresh_button.connect_clicked(move |_| {
        let directory = directory_entry.text().to_string();

        if directory.is_empty() {
            show_error_dialog(&window_clone, "No directory provided.");
            return;
        }

        if let Err(e) = save_directory_to_config(&directory) {
            show_error_dialog(&window_clone, &format!("Failed to save directory to config: {}", e));
            return;
        }

        run_data_pipeline(&directory);

        populate_song_list(&list_store);
    });

    let dark_mode_enabled_clone = Rc::clone(&dark_mode_enabled);
    let settings_clone = settings.clone();
    toggle_theme_button.connect_clicked(move |_| {
        let mut dark_mode = dark_mode_enabled_clone.borrow_mut();
        *dark_mode = !*dark_mode;
        settings_clone.set_gtk_application_prefer_dark_theme(*dark_mode);
    });

    let _window_clone = window.clone();
    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}
