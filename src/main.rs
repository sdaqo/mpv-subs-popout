// Dont Start with Terminal on Windows
#![windows_subsystem = "windows"]

mod config;
mod app;
mod mpv;
mod language;

use std::panic;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt};
use gtk::Application;

use app::build_window;
use app::channel::Message;
use app::utils::load_settings;

fn main() {
    panic::set_hook(Box::new(|_info| {
        println!("{}", _info.to_string());
        // Suppress panic warnings because mpvipc panics when we get an RST packet. ¯\_(ツ)_/¯
    }));
   
    let application = Application::builder()
        .application_id("dev.sdaqo.mpvSubsPopout")
        .build();

    application.connect_activate(|app| {
        let window = build_window(app);

        window.show_all();
        load_settings(&window);

        window.imp().channel_sender.get().unwrap().send(Message::SpawnThread).ok();
    });

    application.run();
}