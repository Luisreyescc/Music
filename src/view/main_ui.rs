use gtk::prelude::*;
use gtk::{TreeView, TreeViewColumn, CellRendererText, Box as GtkBox, Orientation, Window, WindowType, Label, Entry, ScrolledWindow, ListStore};

pub fn build_ui() {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Music Manager");
    window.set_default_size(800, 600);

    let main_box = GtkBox::new(Orientation::Horizontal, 5);

    let song_list_box = GtkBox::new(Orientation::Vertical, 5);

    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    let tree_view = TreeView::new();

    let list_store = ListStore::new(&[glib::Type::STRING, glib::Type::STRING, glib::Type::STRING]);

    let iter = list_store.append();
    list_store.set(&iter, &[(0, &"4:27"), (1, &"Yellow"), (2, &"Coldplay")]);

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

    let details_box = GtkBox::new(Orientation::Vertical, 5);

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

    right_box.set_size_request(300, -1); 
    right_box.pack_start(&details_box, true, true, 0);

    main_box.pack_start(&song_list_box, true, true, 0);
    main_box.pack_start(&right_box, false, false, 0);

    window.add(&main_box);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}
