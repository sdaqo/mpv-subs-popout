use gtk::{gdk, subclass::prelude::ObjectSubclassIsExt, prelude::*};

use crate::config::AppConfig;
use crate::app::window::MpvSubsWindow;

pub fn get_style_string(cfg: &AppConfig) -> Vec<u8> {
    let style_string = format!(
        "window {{ background: {}; }} .sub_label {{ font-size: {}pt; color: {}; font-family: {};  }} ", 
        cfg.bg_col,
        cfg.font_size, 
        cfg.text_col,
        cfg.font_family
    );
    style_string.as_bytes().to_owned()
}

pub fn load_settings(window: &MpvSubsWindow) {
    let config = AppConfig::new();

    window.imp()
        .css_provider
        .get()
        .unwrap()
        .load_from_data(&get_style_string(&config)).ok();

    window.set_keep_above(config.ontop);
    window.set_decorated(config.borders);

    if config.docked {
        window.set_type_hint(gdk::WindowTypeHint::Dock);
    }
} 
