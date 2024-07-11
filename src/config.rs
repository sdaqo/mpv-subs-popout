use home_config::HomeConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TlEngineConfig {
    pub name: String,
    pub api_key: String,
    pub default_lang_from: String,
    pub default_lang_to: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub font_size: i32,
    pub font_family: String,
    pub docked: bool,
    pub ontop: bool,
    pub borders: bool,
    pub bg_col: String,
    pub text_col: String,
    pub size_lock: Option<(i32, i32)>,
    pub strip_nl: bool,
    pub auto_tl: bool,
    pub default_tl_engine: String,
    pub translators: Vec<TlEngineConfig>,
}

impl AppConfig {
    pub fn new() -> Self {
        let config = AppConfig::config_dir();
        let default_config = AppConfig {
            font_size: 13,
            font_family: "Sans".to_owned(),
            docked: false,
            ontop: true,
            borders: true,
            strip_nl: false,
            size_lock: None,
            bg_col: "rgb(42, 46, 50)".to_owned(),
            text_col: "rgb(255, 255, 255)".to_owned(),
            auto_tl: false,
            ..AppConfig::default()
        };

        config.json::<AppConfig>().unwrap_or(default_config)
    }

    pub fn config_dir() -> HomeConfig {
        HomeConfig::with_config_dir("mpv-subs-popout", "config.json")
    }

    pub fn save(&self) {
        let config = AppConfig::config_dir();
        config.save_json(self).unwrap();
    }
}
