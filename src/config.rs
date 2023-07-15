use home_config::HomeConfig;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub font_size: i32,
    pub font_family: String,
    pub docked: bool,
    pub ontop: bool,
    pub borders: bool,
    pub bg_col: String,
    pub text_col: String 
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
            bg_col: "rgb(42, 46, 50)".to_owned(),
            text_col: "rgb(255, 255, 255)".to_owned()
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
