//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with a ListStore.

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn main() {
    let application = gtk::Application::new(Some("com.github.muito_engracado"), Default::default())
        .expect("Initialization failed...");
    application.connect_activate(|app| build_ui(app));
    application.run(&args().collect::<Vec<_>>());
}

fn build_ui(app: &gtk::Application) {
    // Widgets setup

    // Viewers
    let text_view: gtk::TextView = gtk::TextViewBuilder::default().build();
    let tree_view: gtk::TreeView = gtk::TreeViewBuilder::default().build();
    tree_view.set_headers_visible(true);
    append_column(&tree_view, 0, "Source");
    append_column(&tree_view, 1, "Title");

    // Populating treeview
    let model = create_tree_view_storage();
    tree_view.set_model(Some(&model));
    add_row(&model, "Stack Oveflow", "Patricia Tree");
    add_row(&model, "OSS", "Windows 7 goes open source");
    add_row(&model, "Blender.org", "New assets library");

    // Scrollers
    let feed_scroll: gtk::ScrolledWindow = gtk::ScrolledWindowBuilder::default().build();
    feed_scroll.add(&tree_view);
    let text_scroll: gtk::ScrolledWindow = gtk::ScrolledWindowBuilder::default().build();
    text_scroll.add(&text_view);

    // Vertical Pan
    let vertical_paned: gtk::Paned = gtk::Paned::new(gtk::Orientation::Vertical);
    vertical_paned.set_vexpand(true);
    vertical_paned.set_wide_handle(true);
    vertical_paned.set_position(900);
    vertical_paned.add(&feed_scroll);
    vertical_paned.add(&text_scroll);

    // Creation of the label.
    let button = gtk::Button::with_label("Click here");

    // Vbox
    let vertical_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 1);
    vertical_box.add(&vertical_paned);
    vertical_box.add(&button);
    vertical_box.set_vexpand_set(true);

    // Main  window setup
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Muito engraçado");
    window.set_position(gtk::WindowPosition::Center);
    window.add(&vertical_box);
    window.show_all();
}

fn create_tree_view_storage() -> gtk::ListStore {
    gtk::ListStore::new(&[String::static_type(), String::static_type()])
}

fn add_row(model: &gtk::ListStore, source: &str, title: &str) {
    model.insert_with_values(None, &[0, 1], &[&source, &title]);
}

fn append_column(tree: &gtk::TreeView, id: i32, title: &str) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);

    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    column.set_title(title);
    tree.append_column(&column);
}
