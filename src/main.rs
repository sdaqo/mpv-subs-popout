mod config;
mod context_menu;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, CheckButton, StyleContext, CssProvider, FontChooserDialog, Scale};
use gtk::glib;
use gtk::glib::clone;
use gtk::gdk;
use mpvipc::{Mpv, Event, Property, MpvDataType};
use std::thread;
use std::time;
use std::panic;

use config::AppConfig;
use context_menu::ContextMenu;

enum Message {
    UpdateLabel(String),
    SpawnThread(glib::Sender<Message>),
    Quit
}

fn main() {
    panic::set_hook(Box::new(|_info| {
        // Suppress panic warnings because mpvipc panics when we get an RST packet. ¯\_(ツ)_/¯
    }));

    let application = Application::builder()
        .application_id("dev.sdaqo.mpvSubsPopout")
        .build();

    application.connect_activate(|app| {

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Mpv Subs Popout")
            .default_width(350)
            .default_height(70)
            .decorated(false)
            .build(); 

        let config = AppConfig::new(); 

        if config.docked {
            window.set_type_hint(gdk::WindowTypeHint::Dock);
        }

        let provider = CssProvider::new();
        
        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error Initializing screen"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let _ = provider.load_from_data(&get_style_string(config.font_size, config.font_family));

        let sub_label = Label::builder()
            .name("sub_label")
            .build();

        window.add(&sub_label);

        
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        
        receiver.attach(None, clone!(@weak sub_label, @weak app => @default-return glib::Continue(true), move |msg| {
            match msg {
                Message::UpdateLabel(text) => { sub_label.set_text(text.as_str()) },
                Message::SpawnThread(_sender) => { 
                    thread::spawn(move || {
                        let _ = panic::catch_unwind(|| {
                            update_thread_target(_sender.clone());
                        });

                        let _ = _sender.send(Message::SpawnThread(_sender.clone()));
                    });
                },
                Message::Quit => { app.quit() }
            }
            glib::Continue(true)
        }));

        let _ = sender.send(Message::SpawnThread(sender.clone()));

        let context_menu = ContextMenu::new();
        add_context_menu_items(&context_menu, &window, &provider, &sender);
        context_menu.attach_to_window(&window);

        window.show_all();
        window.set_keep_above(config.ontop);
    });
    

    application.run();

}


fn add_context_menu_items(ctx_menu: &ContextMenu, window: &gtk::ApplicationWindow, css_provider: &gtk::CssProvider, sender: &glib::Sender<Message>) {
    let config = AppConfig::new();

    let ontop_btn = CheckButton::builder()
        .label("Always on Top")
        .active(config.ontop)
        .build();

    ctx_menu.add_item(&ontop_btn, Box::new(clone!(@weak window => @default-return Inhibit(true), move |wg, _ev|  {
        let state = wg.is_active();
        
        if state {
            wg.set_active(false);
            window.set_keep_above(false);
        } else {
            wg.set_active(true);
            window.set_keep_above(true);
        }
        
        let mut config = AppConfig::new();
        config.ontop = !state;
        config.save();

        Inhibit(true)
    })));

    let dock_btn = CheckButton::builder()
        .label("Docked")
        .active(config.docked)
        .build();

    ctx_menu.add_item(&dock_btn, Box::new(clone!(@weak window => @default-return Inhibit(true), move |wg, _ev| {
        let state = wg.is_active();
        if state {
            wg.set_active(false);
            window.set_type_hint(gdk::WindowTypeHint::Normal);
        } else {
            wg.set_active(true);
            window.set_type_hint(gdk::WindowTypeHint::Dock);
        }

        let mut config = AppConfig::new();
        config.docked = !state;
        config.save();

        Inhibit(true)
    })));

    let font = Label::new(Some("Change Font"));
    font.set_xalign(0.0);

    ctx_menu.add_item(&font, Box::new(clone!(@weak window, @weak css_provider => @default-return Inhibit(true), move |_wg, _ev| {
        let font_chooser = FontChooserDialog::new(
            Some("Choose a font"),
            Some(&window),
        );
        font_chooser.run();

        if let Some(font_desc) = font_chooser.font_desc() {
            let family = font_desc.family().unwrap_or_default().to_string();
            let size = font_desc.size() / gtk::pango::SCALE;
            let style_str = get_style_string(size, family.clone());
            let _ = css_provider.load_from_data(&style_str);

            let mut cfg = AppConfig::new();
            cfg.font_family = family;
            cfg.font_size = size;
            cfg.save();
        }

        font_chooser.close();
        Inhibit(true)
    })));

    let quit = Label::new(Some("Quit"));
    quit.set_xalign(0.0);
    ctx_menu.add_item(&quit, Box::new(clone!(@strong sender => @default-return Inhibit(true), move |_wg, _ev| {
        let _ = sender.send(Message::Quit);
        Inhibit(true)
    })));
}



fn get_sub_text(mpv: &mut Mpv) -> Result<Option<String>, ()> {
    let event: Event;
    match mpv.event_listen() {
        Ok(ev) => { event = ev },
        Err(..) => {
            return Err(());
        }
    }

    if let Event::PropertyChange { id: _, property } = event {
        match property {
            Property::Unknown { name: _, data } => {
                match data {
                    MpvDataType::String(string) => {
                        return Ok(Some(string));
                    },
                    _=> {}
                }
            },
            _=> {}
        }
    }

    Ok(None)
}

fn update_thread_target(sender: glib::Sender<Message>) {
    let _ = sender.send(Message::UpdateLabel("Waiting for an MPV instance.".to_owned()));

    let mut mpv_conn: Mpv;
    loop {
        match  Mpv::connect("/tmp/mpvsock") {
            Ok(mpv) => {
                let _ = sender.send(Message::UpdateLabel("Connected to an MPV instance! Subs will be displayed here.".to_owned()));
                mpv_conn = mpv;

                break;
            },
            Err(..) => {}
        }

        thread::sleep(time::Duration::from_secs(1));
    }

    mpv_conn.observe_property(1, "sub-text").expect("Failed to observe property! ");

    loop {
        let sub_text: Option<String>;

        match get_sub_text(&mut mpv_conn) {
            Ok(text) => { sub_text = text; },
            Err(..) => { break; } 
        }

        match sub_text {
            Some(text) => {
                let _ = sender.send(Message::UpdateLabel(text));
            },
            None => {}
        }
    }
}

fn get_style_string(font_size: i32, font_family: String) -> Vec<u8> {
    let style_string = format!(
        "#sub_label {{ font-size: {}pt; font-family: {} }}", font_size, font_family
    );
    style_string.as_bytes().to_owned()
}
