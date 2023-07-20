mod imp;

use gtk::glib;

glib::wrapper! { 
    pub struct ReferenceDialog(ObjectSubclass<imp::ReferenceDialog>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Bin, gtk::Container, gtk::Widget,
        @implements gtk::Buildable;
}

impl ReferenceDialog {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}