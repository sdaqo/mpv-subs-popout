// Dont Start with Terminal on Windows
#![windows_subsystem = "windows"]

mod config;
mod app;
mod mpv;
mod language;

use std::panic;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, gio};
use gtk::Application;

use app::build_window;
use app::channel::Message;
use app::utils::load_settings;

fn main() {
    panic::set_hook(Box::new(|_info| {
        // Suppress panic warnings because mpvipc panics when we get an RST packet. ¯\_(ツ)_/¯
    }));

    gio::resources_register_include!("mpv_subs_popout.gresources")
        .expect("Failed to register resources.");

    let application = Application::builder()
        .application_id("org.sdaqo.mpv-subs-popout")
        .build();

    application.connect_activate(|app| {
        let window = build_window(app);

        window.set_visual(
            GtkWindowExt::screen(&window)
            .unwrap()
            .rgba_visual().as_ref()
        );
        window.show_all();
        load_settings(&window);

        window.imp().channel_sender.get().unwrap().send(Message::SpawnThread).ok();
    });

    application.run();
}
