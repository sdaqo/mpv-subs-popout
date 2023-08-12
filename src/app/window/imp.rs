use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::glib::once_cell::sync::OnceCell;

use crate::app::channel::Message;

#[derive(Default)]
pub struct MpvSubsWindow {
    pub css_provider: OnceCell<gtk::CssProvider>,
    pub sub_label: OnceCell<gtk::Label>,
    pub tl_label: OnceCell<gtk::Label>,
    pub label_box: OnceCell<gtk::Box>,
    pub channel_sender: OnceCell<glib::Sender<Message>>
}

#[glib::object_subclass]
impl ObjectSubclass for MpvSubsWindow {
    const NAME: &'static str = "MpvSubsWindow";
    type Type = super::MpvSubsWindow;
    type ParentType = gtk::ApplicationWindow;
}


impl ObjectImpl for MpvSubsWindow {}

impl WidgetImpl for MpvSubsWindow {}

impl ContainerImpl for MpvSubsWindow {}

impl BinImpl for MpvSubsWindow {}

impl WindowImpl for MpvSubsWindow {}

impl ApplicationWindowImpl for MpvSubsWindow {}
