use std::collections::HashMap;
use reqwest;
use serde_json;

use crate::language::{LanguageExt, Translator, TranslatorResponse, Error};
use super::Language;


pub const BASE_URL: &str = "https://translate.google.com/translate_a/single";

pub struct Google {
}

impl Translator for Google {
   fn new() -> Self {
      Google { } 
   }

   fn translate(&self, text: &str, in_lang: impl LanguageExt, out_lang: impl LanguageExt) -> Result<TranslatorResponse, Error> {
      let in_lang_code = match Language::from_language_code(in_lang.to_language_code()) {
         Some(lang) => { lang.to_language_code() } ,
         None => { return Err( Error::LanguageNotAvailableError ) }
      };


      let out_lang_code = match Language::from_language_code(out_lang.to_language_code()) {
         Some(lang) => { lang.to_language_code() } ,
         None => { return Err( Error::LanguageNotAvailableError ) }
      };

      let client = reqwest::blocking::Client::new();

      let mut form = HashMap::<&str, &str>::new();
      form.insert("sl", in_lang_code);
      form.insert("tl", out_lang_code);
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
            res.json::<serde_json::Value>()
         },
         Err(error) => {
            return Err(Error::GoogleError(error.status().unwrap_or_default().as_u16()))
         }
      };
      
      let json = match json {
         Ok(json) => { json },
         Err(_err) => { 
            println!("{:?}", _err);
            return Err(Error::DeserializationError);
         }
      };

      let translation = json["sentences"][0]["trans"].as_str().unwrap_or_default();

      let alternatives = json["alternative_translations"][0]["alternative"]
         .as_array();

      let alternatives = match alternatives {
         Some(alternatives) => {
            Some(
               alternatives.iter()
                  .map(|val| {
                     val["word_postproc"].as_str().unwrap_or_default().to_string()
                  })
                  .collect()
            )
         },
         None => { None }
      };


      Ok(TranslatorResponse { 
         translation: translation.to_string(), 
         alternatives
      })
   }
}


