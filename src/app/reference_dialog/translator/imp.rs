use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, glib};

// https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk3_macros/derive.CompositeTemplate.html

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/sdaqo/mpv-subs-popout/translator.ui")]
pub struct TranslatorWidget {
    #[template_child(id = "tl_engine_combo")]
    pub tl_engine_combo: TemplateChild<gtk::ComboBoxText>,
    
    #[template_child(id = "tl_engine_default_cb")]
    pub tl_engine_default_cb: TemplateChild<gtk::CheckButton>,

    #[template_child(id = "api_key_field")]
    pub api_key_field: TemplateChild<gtk::Entry>,

    #[template_child(id = "api_key_hint_label")]
    pub api_key_hint_label: TemplateChild<gtk::LinkButton>,
    
    #[template_child(id = "lang_from_combo")]
    pub lang_from_combo: TemplateChild<gtk::ComboBoxText>,

    #[template_child(id = "lang_to_combo")]
    pub lang_to_combo: TemplateChild<gtk::ComboBoxText>,

    #[template_child(id = "lang_from_default_cb")]
    pub lang_from_default_cb: TemplateChild<gtk::CheckButton>,

    #[template_child(id = "lang_to_default_cb")]
    pub lang_to_default_cb: TemplateChild<gtk::CheckButton>,

    #[template_child(id = "translator_tab_btn")]
    pub translator_tab_btn: TemplateChild<gtk::Button>,

    #[template_child(id = "dict_tab_btn")]
    pub dict_tab_btn: TemplateChild<gtk::Button>,

    #[template_child(id = "switch_langs_btn")]
    pub switch_langs_btn: TemplateChild<gtk::Button>,

    #[template_child(id = "translate_btn")]
    pub translate_btn: TemplateChild<gtk::Button>,

    #[template_child(id = "lang_from_field")]
    pub lang_from_field: TemplateChild<gtk::TextView>,

    #[template_child(id = "lang_to_field")]
    pub lang_to_field: TemplateChild<gtk::TextView>,
}


#[glib::object_subclass]
impl ObjectSubclass for TranslatorWidget {
    const NAME: &'static str = "TranslatorWidget";
    type Type = super::TranslatorWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TranslatorWidget {}

impl WidgetImpl for TranslatorWidget {}

impl ContainerImpl for TranslatorWidget {}

impl BoxImpl for TranslatorWidget {}
