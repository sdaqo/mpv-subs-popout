pub mod translators;
pub mod dictionaries;

#[derive(Debug)]
pub enum Error {
   LanguageNotAvailableError,
   DeserializationError,
   GoogleError(u16)
}

#[derive(Debug)]
pub struct TranslatorResponse {
   pub translation: String,
   pub alternatives: Option<Vec<String>>
}

pub trait Translator {
   fn new() -> Self;
   fn translate(&self, text: &str, in_lang: impl LanguageExt, out_lang: impl LanguageExt) -> Result<TranslatorResponse, Error>;
}

pub trait Dictionary {
   fn new() -> Self;
   fn lookup(&self, word: &str);
}

pub trait ApiKey {
   fn set_key(&mut self, key: &str);
   fn get_key(&self) -> Option<String>;
}

pub trait LanguageExt {
   fn to_language_code(&self) -> &'static str;
   fn from_language_code(code: &str) -> Option<Box<Self>>;
}
