extern crate gio;
extern crate gtk;

use gio::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::*;
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, IconSize, Image, Label,
    Menu, MenuBar, MenuItem, WindowPosition,
};

use std::env::args;

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("MenuBar example");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);

    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu = Menu::new();
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");
    let file_item = MenuItem::new();
    let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let file_image = Image::new_from_file("resources/file.png");
    let file_label = Label::new(Some("File"));
    let folder_item = MenuItem::new();
    let folder_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let folder_image = Image::new_from_icon_name("folder-music-symbolic", IconSize::Menu.into());
    let folder_label = Label::new(Some("Folder"));
    let check_item = CheckMenuItem::new_with_label("Click me!");

    file_box.pack_start(&file_image, false, false, 0);
    file_box.pack_start(&file_label, true, true, 0);
    file_item.add(&file_box);
    folder_box.pack_start(&folder_image, false, false, 0);
    folder_box.pack_start(&folder_label, true, true, 0);
    folder_item.add(&folder_box);
    menu.append(&file_item);
    menu.append(&folder_item);
    menu.append(&check_item);
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    let other_menu = Menu::new();
    let sub_other_menu = Menu::new();
    let other = MenuItem::new_with_label("Another");
    let sub_other = MenuItem::new_with_label("Sub another");
    let sub_other2 = MenuItem::new_with_label("Sub another 2");
    let sub_sub_other2 = MenuItem::new_with_label("Sub sub another 2");
    let sub_sub_other2_2 = MenuItem::new_with_label("Sub sub another2 2");

    sub_other_menu.append(&sub_sub_other2);
    sub_other_menu.append(&sub_sub_other2_2);
    sub_other2.set_submenu(Some(&sub_other_menu));
    other_menu.append(&sub_other);
    other_menu.append(&sub_other2);
    other.set_submenu(Some(&other_menu));
    menu_bar.append(&other);

    let window_weak = window.downgrade();
    quit.connect_activate(move |_| {
        let window = upgrade_weak!(window_weak);
        window.destroy();
    });

    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    // It isn't available directly through gdk::ModifierType, since it has
    // different values on different platforms.
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let label = Label::new(Some("MenuBar example"));

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&label, true, true, 0);
    window.add(&v_box);
    window.show_all();

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["gtk-rs developers"]);
        p.set_website_label(Some("gtk-rs"));
        p.set_website(Some("http://gtk-rs.org"));
        p.set_authors(&["Gtk-rs developers"]);
        p.set_title("About!");
        p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    });
    check_item.connect_toggled(|w| {
        w.set_label(if w.get_active() {
            "Checked"
        } else {
            "Unchecked"
        });
    });
}

fn main() {
    let application = gtk::Application::new("com.github.menu_bar",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
