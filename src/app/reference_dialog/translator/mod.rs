mod imp;

use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};
use glib::clone;
use gtk::prelude::*;
use open;

use crate::language::{prelude::*, translators::google};
use crate::config::{AppConfig, TlEngineConfig};

glib::wrapper! { 
    pub struct TranslatorWidget(ObjectSubclass<imp::TranslatorWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}


impl Default for TranslatorWidget {
    fn default() -> Self {
       Self::new() 
    }
}

impl TranslatorWidget {
    pub fn new() -> Self {
       let obj: TranslatorWidget = glib::Object::builder().build();
       obj.load_translators();
       obj.bind_callbacks();
       obj.load_from_config();
       obj
    }

    pub fn set_text_and_tl(&self, text: &str) {
        self.imp().lang_from_field.buffer().unwrap().set_text(text);
        self.translate();
    }


    fn load_translators(&self) {
        self.imp().tl_engine_combo.append(Some("google_api_v1"), &google::GoogleApiV1::get_name());
        self.imp().tl_engine_combo.append(Some("google_api_v2"), &google::GoogleApiV2::get_name());
        self.imp().tl_engine_combo.append(Some("google_scrape"), &google::GoogleScrape::get_name());
    }

    fn load_stuff_for_translator(&self) {
        let tl_engine_id = self.imp().tl_engine_combo.active_id().unwrap_or_default();

        let (lang_iter, key_get_link) = match tl_engine_id.as_str() {
            "google_api_v1" => {
                (google::Language::get_iterator(), google::GoogleApiV1::get_api_key_url())
            },
            "google_api_v2" => {
                (google::Language::get_iterator(), google::GoogleApiV2::get_api_key_url())
            },
            "google_scrape" => {
                (google::Language::get_iterator(), google::GoogleScrape::get_api_key_url())
            },
            _ => { return; }
        };

        self.imp().lang_from_combo.remove_all();
        self.imp().lang_to_combo.remove_all();
        
        for lang in lang_iter {
            self.imp().lang_from_combo.append(
                Some(&lang.to_language_code()), &lang.to_language_name()
            );
            self.imp().lang_to_combo.append(
                Some(&lang.to_language_code()), &lang.to_language_name()
            );
        }

        if let Some(api_key_url) = key_get_link {
            self.imp().api_key_hint_label.set_label("Get Api Key Here");
            self.imp().api_key_hint_label.set_uri(&api_key_url);
            self.imp().api_key_field.set_sensitive(true);
        } else {
            self.imp().api_key_hint_label.set_label("No Api Key Required");
            self.imp().api_key_hint_label.set_uri("");
            self.imp().api_key_field.set_sensitive(false);
        }

        let mut cfg = AppConfig::new();

        let tl_cfg = cfg.translators
            .iter()
            .find(|t| t.name == tl_engine_id);

        if let Some(tl_cfg) = tl_cfg {
            self.load_from_tl_cfg(tl_cfg);
        } else {
            // Create the config! :) And then call load_from_tl_config (this also means all the creations of the configs are unnecaccary :)))))
            let new_tl_cfg = TlEngineConfig {
                name: tl_engine_id.to_string(),
                ..TlEngineConfig::default()
            };
            self.load_from_tl_cfg(&new_tl_cfg);
            cfg.translators.push(new_tl_cfg);
            cfg.save();
        }

        self.imp().tl_engine_default_cb.set_active(cfg.default_tl_engine == tl_engine_id);

        self.show_all();
    }

    fn load_from_config(&self) {
        let config = AppConfig::new();
        self.imp().tl_engine_combo.set_active_id(Some(&config.default_tl_engine));
        self.load_stuff_for_translator();
    }

    fn load_from_tl_cfg(&self, tl_cfg: &TlEngineConfig) {
        self.imp().api_key_field.buffer().set_text(&tl_cfg.api_key);
        self.imp().lang_from_combo.set_active_id(Some(&tl_cfg.default_lang_from));
        self.imp().lang_to_combo.set_active_id(Some(&tl_cfg.default_lang_to));
        self.imp().lang_from_default_cb.set_active(true);
        self.imp().lang_to_default_cb.set_active(true);
    }

    fn bind_callbacks(&self) {
        let self_clone: TranslatorWidget = self.clone();
        self.imp().tl_engine_combo.connect_changed(clone!(@weak self_clone => move |wg| self_clone.change_tl_engine(wg)));
        self.imp().tl_engine_default_cb.connect_toggled(clone!(@weak self_clone => move |wg| self_clone.default_tl_engine(wg)));
        self.imp().api_key_field.connect_changed(clone!(@weak self_clone => move|wg| self_clone.change_api_key(wg)));
        self.imp().lang_from_default_cb.connect_toggled(clone!(@weak self_clone => move |wg| self_clone.default_lang_from(wg)));
        self.imp().lang_to_default_cb.connect_toggled(clone!(@weak self_clone => move |wg| self_clone.default_lang_to(wg)));
        self.imp().lang_from_combo.connect_changed(clone!(@weak self_clone => move |wg| self_clone.change_lang_from(wg)));
        self.imp().lang_to_combo.connect_changed(clone!(@weak self_clone => move |wg| self_clone.change_lang_to(wg)));
        self.imp().switch_langs_btn.connect_clicked(clone!(@weak self_clone => move |wg| self_clone.switch_langs(wg)));
        self.imp().translate_btn.connect_clicked(clone!(@weak self_clone => move |_wg| self_clone.translate(); ()));
        self.imp().dict_tab_btn.connect_clicked(clone!(@weak self_clone => move |wg| self_clone.change_to_dict(wg)));
        self.imp().api_key_hint_label.connect_clicked(clone!(@weak self_clone => move |wg| self_clone.open_api_get_page(wg)));
    }

    fn change_lang_from(&self, wg: &gtk::ComboBoxText) {
        let cfg = AppConfig::new();
        let lang_id = if let Some(id) = wg.active_id() { id } else { return; };


        if let Some(id) = self.imp().tl_engine_combo.active_id() {
            if let Some(tl_cfg) = cfg.translators.iter().find(|t| t.name == id) {
                self.imp().lang_from_default_cb.set_active(tl_cfg.default_lang_from == lang_id);
            }
        }
    }
    
    fn change_lang_to(&self,  wg: &gtk::ComboBoxText) {
        let cfg = AppConfig::new();
        let lang_id = if let Some(id) = wg.active_id() { id } else { return; };


        if let Some(id) = self.imp().tl_engine_combo.active_id() {
            if let Some(tl_cfg) = cfg.translators.iter().find(|t| t.name == id) {
                self.imp().lang_to_default_cb.set_active(tl_cfg.default_lang_to == lang_id);
            }
        }
    }

    fn change_tl_engine(&self, _wg: &gtk::ComboBoxText) {
        self.load_stuff_for_translator();
    }

    fn default_tl_engine(&self, wg: &gtk::CheckButton) {
        if !wg.is_active() {
            return;
        }

        let mut cfg = AppConfig::new();
        if let Some(id) = self.imp().tl_engine_combo.active_id() {
            cfg.default_tl_engine = id.to_string();
            cfg.save();
        }
    } 

    fn change_api_key(&self, wg: &gtk::Entry) {
        let mut cfg = AppConfig::new();
        if let Some(id) = self.imp().tl_engine_combo.active_id() {
            if let Some(tl_cfg) = cfg.translators.iter_mut().find(|t| t.name == id) {
                tl_cfg.api_key = wg.buffer().text().to_string();
                cfg.save();
            }
        }
    }

    fn open_api_get_page(&self, wg: &gtk::LinkButton) {
        if let Some(uri) = wg.uri() {
            if uri.is_empty() {
                return;
            }
            let _ = open::that(uri);
        }
    }

    fn default_lang_from(&self, wg: &gtk::CheckButton) {
        if !wg.is_active() {
            return;
        }

        let lang_id = if let Some(id ) = self.imp().lang_from_combo.active_id() { id } else { return; };
        let tl_engine = if let Some(id) = self.imp().tl_engine_combo.active_id() { id } else { return };

        let mut cfg = AppConfig::new();
        if let Some(tl_cfg) = cfg.translators.iter_mut().find(|t| t.name == tl_engine) {
            tl_cfg.default_lang_from = lang_id.to_string();
            cfg.save();
        }
    }

    fn default_lang_to(&self, wg: &gtk::CheckButton) {
        if !wg.is_active() {
            return;
        }

        let lang_id = if let Some(id ) = self.imp().lang_to_combo.active_id() { id } else { return; };
        let tl_engine = if let Some(id) = self.imp().tl_engine_combo.active_id() { id } else { return };


        let mut cfg = AppConfig::new();

        if let Some(tl_cfg) = cfg.translators.iter_mut().find(|t| t.name == tl_engine) {
            tl_cfg.default_lang_to = lang_id.to_string();
            cfg.save();
        }
    }

    fn switch_langs(&self, _wg: &gtk::Button) {
        if let Some(to_id) = self.imp().lang_to_combo.active_id() {
            if let Some(from_id) = self.imp().lang_from_combo.active_id() {
                self.imp().lang_from_combo.set_active_id(Some(to_id.as_str())); 
                self.imp().lang_to_combo.set_active_id(Some(from_id.as_str()));
                self.imp().lang_from_default_cb.set_active(self.imp().lang_to_default_cb.is_active());
                self.imp().lang_to_default_cb.set_active(self.imp().lang_from_default_cb.is_active());
            }
        }
    }

    fn translate(&self) -> Option<()> {
        let buffer = self.imp().lang_from_field.buffer()?;
        let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true)?;
        let lang_from = self.imp().lang_from_combo.active_id()?.to_string();
        let lang_to = self.imp().lang_to_combo.active_id()?.to_string();
        let translator = self.imp().tl_engine_combo.active_id()?.to_string();
        let api_key = self.imp().api_key_field.buffer().text();

        let translatet_text = match translator.as_str() {
            "google_api_v1" => {
               google::GoogleApiV1::new().translate(
                   &text, *google::Language::from_language_code(&lang_from)?, *google::Language::from_language_code(&lang_to)?
               )
            },
            "google_api_v2" => {
                let mut google_api_v2 = google::GoogleApiV2::new();
                google_api_v2.set_key(api_key);
                google_api_v2.translate(
                    &text, *google::Language::from_language_code(&lang_from)?, *google::Language::from_language_code(&lang_to)?
                )
            },
            "google_scrape" => {
                google::GoogleScrape::new().translate(
                    &text, *google::Language::from_language_code(&lang_from)?, *google::Language::from_language_code(&lang_to)?
                )
            },
            _ => { return Some(()); }
        };

        match translatet_text {
            Ok(res) => {
                let mut text = res.translation;
                if let Some(alternatives) = res.alternatives {
                    text.push_str("\n\nAlternatives:\n");
                    for alt in alternatives {
                        text.push_str(&format!("{}\n", alt));
                    }
                }

                self.imp().lang_to_field.buffer()?.set_text(&text);
            },
            Err(e) => {
                self.imp().lang_to_field.buffer()?.set_text(&format!("Error: {:?}", e));
            }
        } 

        Some(())
    }

    fn change_to_dict(&self, _wg: &gtk::Button) {
        // ** NOT IMPLEMENTED **
    } 
}
