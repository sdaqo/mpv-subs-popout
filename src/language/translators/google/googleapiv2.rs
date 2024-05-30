use std::collections::BTreeMap;

use reqwest;
use serde_json;

use super::get_language_map;
use crate::language::{prelude::*, Error, TranslatorResponse};

const BASE_URL: &str = "https://translation.googleapis.com/language/translate/v2";

pub struct GoogleApiV2 {
    url: String,
    key: Option<String>,
}

impl Translator for GoogleApiV2 {
    fn new(url_override: Option<String>) -> Self {
        if let Some(x) = url_override {
            GoogleApiV2 { url: x, key: None }
        } else {
            GoogleApiV2 {
                url: BASE_URL.to_string(),
                key: None,
            }
        }
    }

    fn translate(
        &self,
        text: &str,
        in_lang_code: &str,
        out_lang_code: &str,
    ) -> Result<TranslatorResponse, Error> {
        let client = reqwest::blocking::Client::new();

        let res = client
            .post(&self.url)
            .query(&[
                ("format", "text"),
                ("target", &out_lang_code),
                ("source", &in_lang_code),
                ("key", self.key.as_ref().unwrap()),
                ("q", text),
            ])
            .send();

        let json = match res {
            Ok(res) => {
                if res.status().as_u16() != 200 {
                    return Err(Error::Google(res.status().as_u16()));
                }
                res.json::<serde_json::Value>()
            }
            Err(error) => {
                return Err(Error::Request(error.to_string()));
            }
        };

        let json = match json {
            Ok(json) => json,
            Err(error) => {
                return Err(Error::Deserialization(error.to_string()));
            }
        };

        let translation = json["data"]["translations"][0]["translatedText"].as_str();

        match translation {
            Some(trans) => Ok(TranslatorResponse {
                translation: trans.to_string(),
                alternatives: None,
            }),
            None => Err(Error::NoTranslation),
        }
    }

    fn get_url(&self) -> String {
        self.url.to_string()
    }

    fn get_name() -> String {
        "Google Translate Api V2".to_string()
    }

    fn get_api_key_url() -> Option<String> {
        Some(
            "https://console.cloud.google.com/marketplace/product/google/translate.googleapis.com"
                .to_string(),
        )
    }

    fn get_language_map() -> BTreeMap<&'static str, &'static str> {
        get_language_map()
    }
}

impl ApiKey for GoogleApiV2 {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }
}
