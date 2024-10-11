use gtk::prelude::*;
use gtk::{ProgressBar, Button, TreeView, TreeViewColumn, CellRendererText, Box as GtkBox, 
    Orientation, Window, WindowType, Label, Entry, ScrolledWindow, ListStore, Settings, MenuButton, Popover, Frame};
use crate::controller::controller::{populate_song_list, save_directory_to_config, 
    create_database_connection, show_error_dialog, get_song_details, remove_database_file_if_exists, 
    extract_songs_from_directory, insert_song_into_database, create_tables_if_not_exist};
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
    window.set_default_size(900, 600);

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

    let content_box = GtkBox::new(Orientation::Horizontal, 10);

    let song_list_box = GtkBox::new(Orientation::Vertical, 5);
    let directory_entry = Entry::new();
    directory_entry.set_placeholder_text(Some("Enter music directory here..."));
    song_list_box.pack_start(&directory_entry, false, false, 5);

    let refresh_button = Button::with_label("Refresh");
    song_list_box.pack_start(&refresh_button, false, false, 0);

    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    let tree_view = TreeView::new();
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
    song_list_box.pack_start(&scrolled_window, true, true, 5);

    let progress_bar = ProgressBar::new();
    song_list_box.pack_start(&progress_bar, false, false, 5);

    let right_box = GtkBox::new(Orientation::Vertical, 10);

    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("Search ..."));
    right_box.pack_start(&search_entry, false, false, 0);

    let details_frame = Frame::new(Some("Song Details"));
    let details_box = GtkBox::new(Orientation::Vertical, 10);
    details_box.set_margin_top(10);
    details_box.set_margin_bottom(10);
    details_box.set_margin_start(10);
    details_box.set_margin_end(10);

    let label_title = Label::new(Some("Title: "));
    let label_path = Label::new(Some("Path: "));
    let label_track = Label::new(Some("Track number: "));
    let label_year = Label::new(Some("Year: "));
    let label_genre = Label::new(Some("Genre: "));

    details_box.pack_start(&label_title, false, false, 5);
    details_box.pack_start(&label_path, false, false, 5);
    details_box.pack_start(&label_track, false, false, 5);
    details_box.pack_start(&label_year, false, false, 5);
    details_box.pack_start(&label_genre, false, false, 5);

    details_frame.add(&details_box);
    right_box.pack_start(&details_frame, true, true, 0);

    content_box.pack_start(&song_list_box, true, true, 0);
    content_box.pack_start(&right_box, false, false, 0);

    main_box.pack_start(&header_box, false, false, 0);
    main_box.pack_start(&content_box, true, true, 0);

    window.add(&main_box);
    window.show_all();

    let window_clone = window.clone();

    refresh_button.connect_clicked(move |_| {
        if let Err(e) = remove_database_file_if_exists() {
            eprintln!("Error: Could not remove existing database file: {}", e);
            show_error_dialog(&window_clone, &format!("Error: {}", e));
            return;
        }

        let directory = directory_entry.text().to_string();

        if directory.is_empty() {
            eprintln!("Error: No directory provided.");
            show_error_dialog(&window_clone, "No directory provided.");
            return;
        }

        if let Err(e) = save_directory_to_config(&directory) {
            eprintln!("Failed to save directory to config: {}", e);
            show_error_dialog(&window_clone, &format!("Failed to save directory to config: {}", e));
            return;
        }

        let extracted_data = extract_songs_from_directory(&directory);
        let total_songs = extracted_data.len() as f64;

        if total_songs == 0.0 {
            eprintln!("No songs found in the specified directory.");
            show_error_dialog(&window_clone, "No songs found in the specified directory.");
            return;
        }
        
        let connection = match create_database_connection() {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("Failed to connect to the database: {}", err);
                show_error_dialog(&window_clone, &format!("Failed to connect to the database: {}", err));
                return;
            }
        };

        if let Err(e) = create_tables_if_not_exist(&connection) {
            eprintln!("Failed to create tables in the database: {}", e);
            show_error_dialog(&window_clone, &format!("Failed to create tables in the database: {}", e));
            return;
        }

        let mut processed_songs = 0.0;
        for tag_map in extracted_data {
            match insert_song_into_database(&connection, tag_map) {
                Ok(_) => {
                    processed_songs += 1.0;

                    let progress = processed_songs / total_songs;
                    progress_bar.set_fraction(progress);
                    progress_bar.set_text(Some(&format!("{:.0}% Complete", progress * 100.0)));

                    while gtk::events_pending() {
                        gtk::main_iteration();
                    }
                }
                Err(e) => {
                    eprintln!("Failed to insert song into database: {}", e);
                    show_error_dialog(&window_clone, &format!("Failed to insert song into database: {}", e));
                }
            }
        }

        populate_song_list(&list_store);
    });

    let label_title_clone = label_title.clone();
    let label_path_clone = label_path.clone();
    let label_track_clone = label_track.clone();
    let label_year_clone = label_year.clone();
    let label_genre_clone = label_genre.clone();

    tree_view.connect_cursor_changed(move |tree_view| {
        if let Some((model, iter)) = tree_view.selection().selected() {
            let title: String = model.value(&iter, 0).get().unwrap();
            if let Ok(song_details) = get_song_details(&title) {
                label_title_clone.set_text(&format!("Title: {}", song_details.title));
                label_path_clone.set_text(&format!("Path: {}", song_details.path));
                label_track_clone.set_text(&format!("Track number: {}", song_details.track_number));
                label_year_clone.set_text(&format!("Year: {}", song_details.year));
                label_genre_clone.set_text(&format!("Genre: {}", song_details.genre));
            }
        }
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
