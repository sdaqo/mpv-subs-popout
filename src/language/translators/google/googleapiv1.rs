use std::collections::HashMap;
use reqwest;
use serde_json;

use crate::language::{prelude::*, TranslatorResponse, Error};
use super::Language;


const BASE_URL: &str = "https://translate.google.com/translate_a/single";

pub struct GoogleApiV1 {}

impl Translator for GoogleApiV1 {
   fn new() -> Self where Self: Sized {
      GoogleApiV1 { } 
   }

   fn translate(&self, text: &str, in_lang: impl LanguageExt, out_lang: impl LanguageExt) -> Result<TranslatorResponse, Error> {
      let in_lang_code = match Language::from_language_code(&in_lang.to_language_code()) {
         Some(lang) => { lang.to_language_code() } ,
         None => { return Err( Error::LanguageNotAvailable(in_lang.to_language_name()) ) }
      };


      let out_lang_code = match Language::from_language_code(&out_lang.to_language_code()) {
         Some(lang) => { lang.to_language_code() } ,
         None => { return Err( Error::LanguageNotAvailable(out_lang.to_language_name()) ) }
      };

      let client = reqwest::blocking::Client::new();

      let mut form = HashMap::<&str, &str>::new();
      form.insert("sl", &in_lang_code);
      form.insert("tl", &out_lang_code);
      form.insert("q",  text);

      let res = client.post(BASE_URL)
         .query(&[
            ("client", "gtx"),
            ("dt", "t"),  // with translation
            ("dt", "at"), // with alternative translation
            ("dj", "1")   // with proper json
         ])
         .header(
            reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded"
         )
         .form(&form)
         .send();
      
      let json = match res {
         Ok(res) => {
            if res.status().as_u16() != 200 {
               return Err(Error::Google(res.status().as_u16()));
            }
            res.json::<serde_json::Value>()
         },
         Err(error) => {
            return Err(Error::Request(error.to_string()));
         }
      };
      
      let json = match json {
         Ok(json) => { json },
         Err(error) => { 
            return Err(Error::Deserialization(error.to_string()));
         }
      };

      let translation = json["sentences"][0]["trans"].as_str();

      let translation = match translation {
          Some(trans) => {trans},
          None => {return  Err(Error::NoTranslation);}
      };

      let alternatives = json["alternative_translations"][0]["alternative"]
         .as_array();
      
      let alternatives = alternatives.map(|alternatives| alternatives.iter()
         .map(|val| {
            val["word_postproc"].as_str().unwrap_or_default().to_string()
         })
         .collect());


      Ok(TranslatorResponse { 
         translation: translation.to_string(), 
         alternatives
      })
   }

   fn get_name() -> String {
       "Google Translate Api V1".to_string()
   }


   fn get_api_key_url() -> Option<String> {
      None
   }
}
