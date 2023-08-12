pub mod window;
pub mod ctxmenu;
pub mod utils;
pub mod channel;
pub mod reference_dialog;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, Label};
use gtk::gdk;

use window::MpvSubsWindow;
use ctxmenu::build_ctxmenu;
use channel::setup_channel;
use crate::config::AppConfig;


pub fn build_window(app: &gtk::Application) -> MpvSubsWindow {
    let window = MpvSubsWindow::new(app);

    window.set_title("Mpv Subs Popout");
    window.set_default_height(70);
    window.set_default_width(350);

    window.imp().css_provider.set(gtk::CssProvider::new()).ok();

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error Initializing screen"),
        window.imp().css_provider.get().unwrap(),
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let ctx_menu = build_ctxmenu(&window);
    
    let label_box = gtk::Box::new(gtk::Orientation::Vertical, 6);

    label_box.set_homogeneous(true);

    let sub_label = Label::builder()
        .name("sub_label")
        .selectable(true)
        .build();

    sub_label.style_context().add_class("sub_label");

    ctx_menu.attach_to_widget(&sub_label);

    window.imp().sub_label.set(sub_label).ok();
    label_box.add(window.imp().sub_label.get().unwrap());
    
    let cfg = AppConfig::new();

    let tl_label = Label::builder()
        .name("tl_label")
        .build();

    tl_label.style_context().add_class("sub_label");

    ctx_menu.attach_to_widget(&tl_label);

    window.imp().tl_label.set(tl_label).ok();

    if cfg.auto_tl {
        label_box.add(window.imp().tl_label.get().unwrap());
    }


    window.add(&label_box);
    window.imp().label_box.set(label_box).ok();

    window.imp().channel_sender.set(setup_channel(&window)).ok();
    
    window
}

