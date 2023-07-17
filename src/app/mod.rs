pub mod window;
pub mod ctxmenu;
pub mod utils;
pub mod channel;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, Label};
use gtk::gdk;

use window::MpvSubsWindow;
use ctxmenu::build_ctxmenu;
use channel::setup_channel;


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
    ctx_menu.attach_to_window(&window);

    // TODO: Custom Label with useful functionality
    window.imp().sub_label.set(Label::builder().name("sub_label").build()).ok();
    window.add(window.imp().sub_label.get().unwrap());

    window.imp().channel_sender.set(setup_channel(&window)).ok();

    return window;
}

