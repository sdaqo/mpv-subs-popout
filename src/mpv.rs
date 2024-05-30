use gtk::glib;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use crate::app::channel::Message;
use crate::config::*;
use crate::language::prelude::ApiKey;
use crate::language::translators::meta::MetaTranslator;

use mpvipc::{Event, Mpv, MpvDataType, Property};

pub fn mpv_subs_update(sender: glib::Sender<Message>) {
    sender
        .send(Message::UpdateLabel(
            "Waiting for an MPV instance.".to_owned(),
        ))
        .ok();
    sender
        .send(Message::UpdateTlLabel(
            "Waiting for an MPV instance.".to_owned(),
        ))
        .ok();

    #[cfg(target_os = "linux")]
    let server_path = "/tmp/mpvsock";

    #[cfg(target_os = "windows")]
    let server_path = "\\\\.\\pipe\\mpvsock";

    let mut mpv_conn: Mpv;
    loop {
        if let Ok(mpv) = Mpv::connect(server_path) {
            sender
                .send(Message::UpdateLabel(
                    "Connected to an MPV instance! Subs will be displayed here.".to_owned(),
                ))
                .ok();
            mpv_conn = mpv;

            break;
        }

        thread::sleep(time::Duration::from_secs(1));
    }

    mpv_conn
        .observe_property(1, "sub-text")
        .expect("Failed to observe property! ");

    let mut cancel_token = Arc::new(Mutex::new(false));
    while let Ok(sub_text) = get_sub_text(&mut mpv_conn) {
        if let Some(text) = sub_text {
            sender.send(Message::UpdateLabel(text.clone())).ok();

            let cfg = AppConfig::new();
            if !cfg.auto_tl {
                continue;
            }

            if text.is_empty() {
                sender.send(Message::UpdateTlLabel("".to_string())).ok();
                continue;
            }

            let tl_cfg = cfg
                .translators
                .into_iter()
                .find(|t| t.name == cfg.default_tl_engine);

            if let Some(tl_cfg) = tl_cfg {
                if tl_cfg.default_lang_to.is_empty() || tl_cfg.default_lang_from.is_empty() {
                    sender
                        .send(Message::UpdateTlLabel(
                            "Set Translator Defaults first to use this.".to_string(),
                        ))
                        .ok();
                    continue;
                }

                *cancel_token.lock().unwrap() = false;
                cancel_token = Arc::new(Mutex::new(false));

                let cancel_token_clone = Arc::clone(&cancel_token);
                let sender_clone = sender.clone();
                thread::spawn(move || {
                    tl_and_set(text, tl_cfg, sender_clone, cancel_token_clone);
                });
            } else {
                sender
                    .send(Message::UpdateTlLabel(
                        "Set Translator Defaults first to use this.".to_string(),
                    ))
                    .ok();
                continue;
            }
        }
    }
}

fn tl_and_set(
    text: String,
    tl_engine_config: TlEngineConfig,
    sender: glib::Sender<Message>,
    cancel_token: Arc<Mutex<bool>>,
) -> Option<()> {
    let lang_from = tl_engine_config.default_lang_from;
    let lang_to = tl_engine_config.default_lang_to;

    let mut translator = MetaTranslator::new(tl_engine_config.name, tl_engine_config.url);
    if tl_engine_config.api_key.len() > 0 {
        translator.set_key(tl_engine_config.api_key);
    }
    let translation = translator.translate(&text, &lang_from, &lang_to);

    let tl = match translation {
        Ok(res) => res.translation,
        Err(e) => {
            format!("Error when Translating: {:?}", e)
        }
    };

    if !*cancel_token.lock().unwrap() {
        sender.send(Message::UpdateTlLabel(tl)).ok();
    }

    Some(())
}

fn get_sub_text(mpv: &mut Mpv) -> Result<Option<String>, ()> {
    let event: Event = match mpv.event_listen() {
        Ok(ev) => ev,
        Err(..) => {
            return Err(());
        }
    };

    if let Event::PropertyChange {
        id: _,
        property:
            Property::Unknown {
                name: _,
                data: MpvDataType::String(string),
            },
    } = event
    {
        return Ok(Some(string));
    }

    Ok(None)
}
