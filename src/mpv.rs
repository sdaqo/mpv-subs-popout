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

    mpv_conn
        .observe_property(2, "secondary-sub-text")
        .expect("Failed to observe property! ");

    let mut cancel_token = Arc::new(Mutex::new(false));

    let mut last_sub = String::new();
    let mut last_secondary_sub = String::new();

    let mut has_two_subs = false;

    while let Ok(event) = mpv_conn.event_listen() {
        let cfg = AppConfig::new();

        if let Event::PropertyChange {
            id: 2,
            property:
                Property::Unknown {
                    name: _,
                    data: MpvDataType::String(mut text),
                },
        } = event
        {
            if cfg.strip_nl {
                text = text.replace("\n", " ");
            }

            sender.send(
                Message::UpdateLabel(
                    format!("{}\n\n{}", last_sub.clone(), text.clone())
                )
            ).ok();
            last_secondary_sub = text;
        } else if let Event::PropertyChange {
            id: 1,
            property:
                Property::Unknown {
                    name: _,
                    data: MpvDataType::String(mut text),
                },
        } = event 
        {
            if cfg.strip_nl {
                text = text.replace("\n", " ");
            }

            if !last_secondary_sub.is_empty() {
                has_two_subs = true;
            }
            
            let mut message = match has_two_subs {
                true => format!("{}\n\n{}", text.clone(), last_secondary_sub.clone()),
                false => text.clone()
            };

            sender.send(
                Message::UpdateLabel(
                    message                     
                )
            ).ok();
            last_sub = text.clone();
            
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
    if !tl_engine_config.api_key.is_empty() {
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
