use glib::{MainContext, Sender, clone};
use gtk::{glib, subclass::prelude::ObjectSubclassIsExt, prelude::LabelExt};
use std::thread;
use std::panic;

use crate::app::MpvSubsWindow;
use crate::mpv::mpv_subs_update;

pub enum Message {
    UpdateLabel(String),
    SpawnThread,
    Quit
}

pub fn setup_channel(window: &MpvSubsWindow) -> Sender<Message> {
    let (sender, receiver) = MainContext::channel::<Message>(glib::PRIORITY_DEFAULT);

    receiver.attach(None, clone!(@weak window, @strong sender => @default-return glib::Continue(true), move |msg| {
        match msg {
            Message::UpdateLabel(text) => { 
                window.imp()
                    .sub_label
                    .get()
                    .unwrap()
                    .set_text(text.as_str());
            },

            Message::SpawnThread => { 
                thread::spawn(clone!(@strong sender => move || {
                    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                        mpv_subs_update(sender.clone());
                    }));

                    let _ = sender.send(Message::SpawnThread);

                }));
            },
            Message::Quit => { window.quit() }
        }
        glib::Continue(true)
    }));

    return sender;
}