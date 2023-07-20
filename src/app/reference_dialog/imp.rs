use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, glib, Dialog, Button};


#[derive(Default, CompositeTemplate)]
#[template(resource = "")]
pub struct ReferenceDialog {
    #[template_child]
    pub Button: TemplateChild<Button>
}


#[glib::object_subclass]
impl ObjectSubclass for ReferenceDialog {
    const NAME: &'static str = "ReferenceDialog";
    type Type = super::ReferenceDialog;
    type ParentType = gtk::Dialog;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template_child("button");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ReferenceDialog {}

impl ObjectImpl for ReferenceDialog {}

impl WidgetImpl for ReferenceDialog {}

impl ContainerImpl for ReferenceDialog {}

impl BinImpl for ReferenceDialog {}

impl WindowImpl for ReferenceDialog {}

impl DialogImpl for ReferenceDialog {}