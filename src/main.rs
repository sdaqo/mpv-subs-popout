use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, CheckButton, StyleContext, CssProvider, FontChooserDialog, Scale};
use gtk::glib;
use gtk::gdk;
use mpvipc::{Mpv, Event, Property, MpvDataType};
use serde::{Serialize, Deserialize};
use home_config::HomeConfig;
use std::thread;
use std::time;
use std::panic;

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    font_size: i32,
    font_family: String,
    docked: bool,
    ontop: bool
}

impl AppConfig {
    fn new() -> Self {
        let config = AppConfig::config_dir();
        let default_config = AppConfig {
            font_size: 13,
            font_family: "".to_owned(),
            docked: false,
            ontop: true
        };

        config.json::<AppConfig>().unwrap_or(default_config)   
    }

    fn config_dir() -> HomeConfig {
        HomeConfig::with_config_dir("mpv-subs-popout", "config.json")
    }

    fn save(&self) {
        let config = AppConfig::config_dir();
        config.save_json(self).unwrap();
    }

}

enum Message {
    UpdateLabel(String),
    SpawnThread(glib::Sender<Message>),
    Quit
}

struct ContextMenu {
    menu: gtk::Menu
}


impl ContextMenu {
    fn new() -> Self {
        let menu = gtk::Menu::new();
        menu.connect_popup_menu(move |menu| {
            menu.popup_easy(3, gtk::current_event_time());
            return true;
        });

        Self { menu }
    }

    fn add_item<W: IsA<gtk::Widget>>(&self, widget: &W, callback: Box<dyn Fn(&W, &gdk::EventButton) -> Inhibit>) {
        let item = gtk::MenuItem::new();
        item.add(widget);
        
        let widget_clone = widget.clone();
        item.connect_button_press_event(move |_wg, ev| {
            callback(&widget_clone, ev)
        });

        self.menu.append(&item);
        item.show_all();
    }

    fn attach_to_window(&self, window: &gtk::ApplicationWindow) {
        let cloned_menu = self.menu.clone();

        window.connect_button_press_event(move |_, event| {
            if event.button() == gdk::BUTTON_SECONDARY {
                cloned_menu.popup_easy(event.button(), event.time());
                Inhibit(true)
            } else {
                Inhibit(false)
            }
        });

    }
}

fn add_context_menu_items(ctx_menu: &ContextMenu, window: &gtk::ApplicationWindow, css_provider: &gtk::CssProvider, sender: &glib::Sender<Message>) {
    let config = AppConfig::new();

    let ontop_btn = CheckButton::builder()
        .label("Always on Top")
        .active(config.ontop)
        .build();

    let ontop_btn_window = window.clone();
    ctx_menu.add_item(&ontop_btn, Box::new(move |wg, _ev|  {
        let state = wg.is_active();
        
        if state {
            wg.set_active(false);
            ontop_btn_window.set_keep_above(false);
        } else {
            wg.set_active(true);
            ontop_btn_window.set_keep_above(true);
        }

        let mut config = AppConfig::new();
        config.ontop = !state;
        config.save();

        Inhibit(true)
    }));

    let dock_btn = CheckButton::builder()
        .label("Docked")
        .active(config.docked)
        .build();

    let dock_btn_window = window.clone();
    ctx_menu.add_item(&dock_btn, Box::new(move |wg, _ev| {
        let state = wg.is_active();
        if state {
            wg.set_active(false);
            dock_btn_window.set_type_hint(gdk::WindowTypeHint::Normal);
        } else {
            wg.set_active(true);
            dock_btn_window.set_type_hint(gdk::WindowTypeHint::Dock);
        }

        let mut config = AppConfig::new();
        config.docked = !state;
        config.save();

        Inhibit(true)
    }));

    let font = Label::new(Some("Change Font"));
    font.set_xalign(0.0);
    let font_window = window.clone();
    let font_css_provider = css_provider.clone();
    ctx_menu.add_item(&font, Box::new(move |_wg, _ev| {
        let font_chooser = FontChooserDialog::new(
            Some("Choose a font"),
            Some(&font_window),
        );
        font_chooser.run();

        if let Some(font_desc) = font_chooser.font_desc() {
            let family = font_desc.family().unwrap_or_default().to_string();
            let size = font_desc.size() / gtk::pango::SCALE;
            let style_str = get_style_string(size, family.clone());
            let _ = font_css_provider.load_from_data(&style_str);

            let mut cfg = AppConfig::new();
            cfg.font_family = family;
            cfg.font_size = size;
            cfg.save();
        }

        font_chooser.close();
        Inhibit(true)
    }));

    let quit = Label::new(Some("Quit"));
    quit.set_xalign(0.0);
    let quit_sender = sender.clone();
    ctx_menu.add_item(&quit, Box::new(move |_wg, _ev| {
        let _ = quit_sender.send(Message::Quit);
        Inhibit(true)
    }));
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

        // Sub Display
        let sub_label = Label::builder()
            .name("sub_label")
            .build();

        window.add(&sub_label);

        
        // Sub Update
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        
        let label_clone = sub_label.clone();
        let app_clone = app.clone();
        receiver.attach(None, move |msg| {
            match msg {
                Message::UpdateLabel(text) => { label_clone.set_text(text.as_str()) },
                Message::SpawnThread(_sender) => { 
                    thread::spawn(move || {
                        let _ = panic::catch_unwind(|| {
                            update_thread_target(_sender.clone());
                        });

                        let _ = _sender.send(Message::SpawnThread(_sender.clone()));
                    });
                },
                Message::Quit => { app_clone.quit() }
            }
            glib::Continue(true)
        });
        let _ = sender.send(Message::SpawnThread(sender.clone()));

        // Context Menu
        let context_menu = ContextMenu::new();
        add_context_menu_items(&context_menu, &window, &provider, &sender);
        context_menu.attach_to_window(&window);

        window.show_all();
        window.set_opacity(0.5);
        window.set_keep_above(config.ontop);
    });
    

    application.run();

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
