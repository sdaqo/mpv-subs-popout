use glib::{clone, MainContext, Sender};
use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};
use std::panic;
use std::thread;

use crate::app::MpvSubsWindow;
use crate::mpv::mpv_subs_update;

pub enum Message {
    UpdateLabel(String),
    UpdateTlLabel(String),
    SetTlLabelVisibilty(bool),
    SpawnThread,
    Quit,
}

pub fn setup_channel(window: &MpvSubsWindow) -> Sender<Message> {
    let (sender, receiver) = MainContext::channel::<Message>(glib::PRIORITY_DEFAULT);

    receiver.attach(
        None,
        clone!(@weak window, @strong sender => @default-return glib::Continue(true), move |msg| {
            match msg {
                Message::UpdateLabel(text) => {
                    window.imp()
                        .sub_label
                        .get()
                        .unwrap()
                        .set_text(text.as_str());
                },

                Message::UpdateTlLabel(text) => {
                    window.imp()
                        .tl_label
                        .get()
                        .unwrap()
                        .set_text(text.as_str())
                },

                Message::SetTlLabelVisibilty(visible) => {
                    let label_box = window.imp().label_box.get().unwrap();
                    let contains_tl_label = label_box
                        .children()
                        .len() > 1;


                    if visible {
                        if !contains_tl_label {
                            label_box.add(window.imp().tl_label.get().unwrap());
                            label_box.show_all();
                        }
                    } else if contains_tl_label {
                            label_box.remove(window.imp().tl_label.get().unwrap());
                            label_box.show_all();
                    }
                },

                Message::SpawnThread => {
                    thread::spawn(clone!(@strong sender => move || {
                        panic::catch_unwind(panic::AssertUnwindSafe(|| {
                            mpv_subs_update(sender.clone());
                        })).ok();

                        sender.send(Message::SpawnThread).ok();

                    }));
                },
                Message::Quit => { window.quit() }
            }
            glib::Continue(true)
        }),
    );

    sender
}
