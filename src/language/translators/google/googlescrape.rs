

use reqwest;
use regex::Regex;

use crate::language::{prelude::*, TranslatorResponse, Error};
use super::Language;


const BASE_URL: &str = "https://translate.google.com/m";

pub struct GoogleScrape {
}

impl Translator for GoogleScrape {
    fn new() -> Self {
      GoogleScrape { } 
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

      let res = client.get(BASE_URL)
         .query(&[
            ("sl", in_lang_code),
            ("tl", out_lang_code),
            ("q", text.to_string())
         ])
         .header(
            reqwest::header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36"
         )
         .send();
      
      let text = match res {
         Ok(res) => {
            if res.status().as_u16() != 200 {
               return Err(Error::GoogleError(res.status().as_u16()));
            }
            res.text()
         },
         Err(error) => {
            return Err(Error::RequestError(error.to_string()));
         }
      };
      
      let text = match text {
         Ok(text) => { text },
         Err(error) => { 
            return Err(Error::DeserializationError(error.to_string()));
         }
      };


      let re = Regex::new("result-container\">([^<]+)").unwrap();
      let capture = re.captures(&text);

      let translation = match capture {
        Some(capture) => {
            match capture.get(1) {
                Some(trans) => {
                    trans.as_str()
                },
                None => {return Err(Error::NoTranslation); }
            }
        },
        None => { return Err(Error::NoTranslation); } 
      };

 
      Ok(TranslatorResponse { 
        translation: translation.to_string(),
        alternatives: None
      })
    }

    fn get_name() -> String {
       "Google Translate Scraper".to_string()
    }
}