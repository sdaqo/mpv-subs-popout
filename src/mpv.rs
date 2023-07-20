use gtk::glib;
use std::thread;
use std::time;

use crate::app::channel::Message;
use crate::language::{prelude::*, translators::google};

use mpvipc::{Mpv, Event, Property, MpvDataType};

pub fn mpv_subs_update(sender: glib::Sender<Message>) {
    let _ = sender.send(Message::UpdateLabel("Waiting for an MPV instance.".to_owned()));

    #[cfg(target_os = "linux")]
    let server_path = "/tmp/mpvsock";

    #[cfg(target_os = "windows")]
    let server_path = "\\\\.\\pipe\\mpvsock";

    let mut mpv_conn: Mpv;
    loop {
        match  Mpv::connect(server_path) {
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
                if text.is_empty() {
                    let _ = sender.send(Message::UpdateLabel(text));
                } else {
                    println!("Doing Translation...");
                    let translator = google::GoogleApi::new();
                    let translated_text: String = match translator.translate(&text, google::Language::Automatic, google::Language::German) {
                        Ok(res) => { res.translation },
                        Err(err) => { 
                            println!("{:?}", err);
                            text}
                    };
                    let _ = sender.send(Message::UpdateLabel(translated_text));
                }
            },
            None => {}
        }
    }
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

