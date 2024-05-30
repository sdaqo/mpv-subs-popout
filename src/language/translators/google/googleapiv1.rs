use reqwest;
use serde_json;
use std::collections::{HashMap, BTreeMap};

use super::get_language_map;
use crate::language::{prelude::*, Error, TranslatorResponse};

const BASE_URL: &str = "https://translate.google.com/translate_a/single";

pub struct GoogleApiV1 {
    url: String,
}

impl Translator for GoogleApiV1 {
    fn new(url_override: Option<String>) -> Self {
        if let Some(x) = url_override {
            GoogleApiV1 { url: x }
        } else {
            GoogleApiV1 {
                url: BASE_URL.to_string(),
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

        let mut form = HashMap::<&str, &str>::new();
        form.insert("sl", &in_lang_code);
        form.insert("tl", &out_lang_code);
        form.insert("q", text);

        let res = client
            .post(&self.url)
            .query(&[
                ("client", "gtx"),
                ("dt", "t"),  // with translation
                ("dt", "at"), // with alternative translation
                ("dj", "1"),  // with proper json
            ])
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&form)
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

        let translation = json["sentences"][0]["trans"].as_str();

        let translation = match translation {
            Some(trans) => trans,
            None => {
                return Err(Error::NoTranslation);
            }
        };

        let alternatives = json["alternative_translations"][0]["alternative"].as_array();

        let alternatives = alternatives.map(|alternatives| {
            alternatives
                .iter()
                .map(|val| {
                    val["word_postproc"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string()
                })
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
        "Google Translate Api V1".to_string()
    }

    fn get_api_key_url() -> Option<String> {
        None
    }

    fn get_language_map() -> BTreeMap<&'static str, &'static str> {
        get_language_map()
    }
}
