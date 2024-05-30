use std::collections::BTreeMap;

use regex::Regex;
use reqwest;

use super::get_language_map;
use crate::language::{prelude::*, Error, TranslatorResponse};

const BASE_URL: &str = "https://translate.google.com/m";

pub struct GoogleScrape {
    url: String,
}

impl Translator for GoogleScrape {
    fn new(url_override: Option<String>) -> Self {
        if let Some(x) = url_override {
            GoogleScrape { url: x }
        } else {
            GoogleScrape {
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

        let res = client.get(&self.url)
         .query(&[
            ("sl", in_lang_code),
            ("tl", out_lang_code),
            ("q", text)
         ])
         .header(
            reqwest::header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36"
         )
         .send();

        let text = match res {
            Ok(res) => {
                if res.status().as_u16() != 200 {
                    return Err(Error::Google(res.status().as_u16()));
                }
                res.text()
            }
            Err(error) => {
                return Err(Error::Request(error.to_string()));
            }
        };

        let text = match text {
            Ok(text) => text,
            Err(error) => {
                return Err(Error::Deserialization(error.to_string()));
            }
        };

        let re = Regex::new("result-container\">([^<]+)").unwrap();
        let capture = re.captures(&text);

        let translation = match capture {
            Some(capture) => match capture.get(1) {
                Some(trans) => trans.as_str(),
                None => {
                    return Err(Error::NoTranslation);
                }
            },
            None => {
                return Err(Error::NoTranslation);
            }
        };

        Ok(TranslatorResponse {
            translation: translation.to_string(),
            alternatives: None,
        })
    }

    fn get_url(&self) -> String {
        self.url.to_string()
    }

    fn get_name() -> String {
        "Google Translate Scraper".to_string()
    }

    fn get_api_key_url() -> Option<String> {
        None
    }

    fn get_language_map() -> BTreeMap<&'static str, &'static str> {
        get_language_map()
    }
}
