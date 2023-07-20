use reqwest;
use serde_json;

use crate::language::{prelude::*, TranslatorResponse, Error};
use super::Language;


const BASE_URL: &str = "https://translation.googleapis.com/language/translate/v2";

pub struct GoogleApiV2 {
    key: Option<String>
}

impl Translator for GoogleApiV2 {
    fn new() -> Self {
        GoogleApiV2 { key: None }
    }

    fn translate(&self, text: &str, in_lang: impl LanguageExt, out_lang: impl LanguageExt) -> Result<TranslatorResponse, Error> {
        let in_lang_code = match Language::from_language_code(&in_lang.to_language_code()) {
            Some(lang) => { lang.to_language_code() } ,
            None => { return Err( Error::LanguageNotAvailableError(in_lang.to_language_name()) ) }
        };
   
   
        let out_lang_code = match Language::from_language_code(&out_lang.to_language_code()) {
            Some(lang) => { lang.to_language_code() } ,
            None => { return Err( Error::LanguageNotAvailableError(out_lang.to_language_name()) ) }
        };
        
        
        let client = reqwest::blocking::Client::new();

        let res = client
            .post(BASE_URL)
            .query(&[
                ("format", "text"),
                ("target", &out_lang_code),
                ("source", &in_lang_code),
                ("key", self.key.as_ref().unwrap()),
                ("q", text)
            ])
            .send();
        
        let json = match res {
            Ok(res) => {
                if res.status().as_u16() != 200 {
                    return Err(Error::GoogleError(res.status().as_u16()));
                }
                res.json::<serde_json::Value>()
             },
             Err(error) => {
                return Err(Error::RequestError(error.to_string()));
             }
        };

        let json = match json {
            Ok(json) => { json },
            Err(error) => { 
               return Err(Error::DeserializationError(error.to_string()));
            }
        };

        let translation = json["data"]["translations"][0]["translatedText"].as_str();

        return match translation {
            Some(trans) => {
                Ok(TranslatorResponse { translation: trans.to_string(), alternatives: None })
            },
            None => {Err(Error::NoTranslation)}
        };
    }

    fn get_name() -> String {
        "Google Translate Api V2".to_string()
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