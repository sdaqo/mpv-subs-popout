pub mod translators;
pub mod dictionaries;

#[derive(Debug)]
pub enum Error {
   LanguageNotAvailable(String),
   Deserialization(String),
   Request(String),
   Google(u16),
   NoTranslation
}

#[derive(Debug)]
pub struct TranslatorResponse {
   pub translation: String,
   pub alternatives: Option<Vec<String>>
}

pub mod prelude {
   pub trait Translator {
      fn new() -> Self where Self: Sized;
      fn translate(&self, text: &str, in_lang: impl LanguageExt, out_lang: impl LanguageExt) -> Result<super::TranslatorResponse, super::Error>;
      fn get_name() -> String;
      fn get_api_key_url() -> Option<String>;
   }

   pub trait Dictionary {
      fn new() -> Self;
      fn lookup(&self, word: &str);
   }

   pub trait ApiKey {
      fn set_key(&mut self, key: String);
      fn get_key(&self) -> Option<String>;
   }

   pub trait LanguageExt {
      fn from_language_name(name: &str) -> Option<Box<Self>>;
      fn from_language_code(code: &str) -> Option<Box<Self>>;
      fn to_language_code(&self) -> String;
      fn to_language_name(&self) -> String;
      fn get_iterator() -> Box<dyn Iterator<Item = Self>> ;
   }
}
