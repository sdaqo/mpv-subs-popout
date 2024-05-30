mod imp;

use glib::Object;
use gtk::{gio, glib, prelude::*, Application};

glib::wrapper! {
    pub struct MpvSubsWindow(ObjectSubclass<imp::MpvSubsWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Bin, gtk::Container, gtk::Widget,
        @implements gio::ActionGroup,gtk::Buildable, gio::ActionMap;
}

impl MpvSubsWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn quit(&self) {
        self.property::<Application>("application").quit();
    }
}
