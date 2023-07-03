use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};
use gtk::glib;
use mpvipc::{Mpv, Event, Property, MpvDataType};
use std::thread;
use std::time;
use std::panic;


enum Message{
    UpdateLabel(String),
    SpawnThread(glib::Sender<Message>)
}


fn main(){
    panic::set_hook(Box::new(|_info| {
        // Suppress panic warnings because mpvipc panics when we get an RST packet. ¯\_(ツ)_/¯
    }));

    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .decorated(false)
            .build();
        
        let sub_label = Label::new(Some("Open a file in mpv and the subs will be displayed here."));
        
        window.add(&sub_label);

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        
        let label_clone = sub_label.clone();
        receiver.attach(None, move |msg| {
            match msg {
                Message::UpdateLabel(text) => { label_clone.set_text(text.as_str()) },
                Message::SpawnThread(_sender) => { 
                    thread::spawn(move || {
                        let result = panic::catch_unwind(|| {
                            update_thread_target(_sender.clone());
                        });

                        match result {
                            Ok(_) => { 
                                _sender.send(Message::SpawnThread(_sender.clone()));
                            },
                            Err(_) => {
                                _sender.send(Message::SpawnThread(_sender.clone()));
                            }
                        }
                    });
                }
            }
            glib::Continue(true)
        });

        sender.send(Message::SpawnThread(sender.clone()));

        window.show_all();
        window.set_keep_above(true);
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
