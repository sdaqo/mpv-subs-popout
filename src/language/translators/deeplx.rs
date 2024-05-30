use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
};
use serde_json;
use std::collections::{BTreeMap, HashMap};

use crate::language::{prelude::*, Error, TranslatorResponse};

const BASE_URL: &str = "http://localhost:1188/translate";

pub struct DeeplX {
    url: String,
    key: Option<String>,
}

impl Translator for DeeplX {
    fn new(url_override: Option<String>) -> Self {
        if let Some(x) = url_override {
            DeeplX { url: x, key: None }
        } else {
            DeeplX {
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
        let mut data = HashMap::<&str, &str>::new();
        if in_lang_code != "auto" {
            data.insert("source_lang", &in_lang_code);
        } else {
            data.insert("source_lang", "");
        }

        data.insert("target_lang", &out_lang_code);
        data.insert("text", text);

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        if let Some(key) = &self.key {
            if self.url.contains("/v2/translate") {
                if let Ok(v) = HeaderValue::from_str(&format!("DeepL-Auth-Key {}", key)) {
                    headers.insert(reqwest::header::AUTHORIZATION, v);
                }
            } else {
                if let Ok(v) = HeaderValue::from_str(&format!("Bearer {}", key)) {
                    headers.insert(reqwest::header::AUTHORIZATION, v);
                }
            }
        };

        let res = client.post(&self.url).json(&data).headers(headers).send();
        let json = match res {
            Ok(res) => {
                if res.status().as_u16() != 200 {
                    return Err(Error::DeeplX(res.status().as_u16()));
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

        let translation = json["data"].as_str();

        let translation = match translation {
            Some(tl) => tl,
            None => return Err(Error::NoTranslation),
        };

        let alternatives: Option<Vec<String>> = json["alternatives"].as_array().map(|vec| {
            vec.iter()
                .filter_map(|val| val.as_str().map(|s| s.to_string()))
                .collect()
        });

        Ok(TranslatorResponse {
            translation: translation.to_string(),
            alternatives,
        })
    }

    fn get_url(&self) -> String {
        self.url.to_string()
    }

    fn get_name() -> String {
        "DeeplX".to_string()
    }

    fn get_api_key_url() -> Option<String> {
        Some("https://deeplx.owo.network/".to_string())
    }

    fn get_language_map() -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();

        map.insert("Automatic", "auto");
        map.insert("Arabic", "AR");
        map.insert("Bulgarian", "BG");
        map.insert("Czech", "CS");
        map.insert("Danish", "DA");
        map.insert("German", "DE");
        map.insert("Greek", "EL");
        map.insert("English", "EN");
        map.insert("Spanish", "ES");
        map.insert("Estonian", "ET");
        map.insert("Finnish", "FI");
        map.insert("French", "FR");
        map.insert("Hungarian", "HU");
        map.insert("Indonesian", "ID");
        map.insert("Italian", "IT");
        map.insert("Japanese", "JA");
        map.insert("Korean", "KO");
        map.insert("Lithuanian", "LT");
        map.insert("Latvian", "LV");
        map.insert("Norwegian Bokmål", "NB");
        map.insert("Dutch", "NL");
        map.insert("Polish", "PL");
        map.insert("Portuguese", "PT");
        map.insert("Romanian", "RO");
        map.insert("Russian", "RU");
        map.insert("Slovak", "SK");
        map.insert("Slovenian", "SL");
        map.insert("Swedish", "SV");
        map.insert("Turkish", "TR");
        map.insert("Ukrainian", "UK");
        map.insert("Chinese", "ZH");

        map
    }
}

impl ApiKey for DeeplX {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }
}
