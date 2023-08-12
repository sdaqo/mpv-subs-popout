pub mod googleapiv1;
pub mod googleapiv2;
pub mod googlescrape;

use strum_macros;
use strum::{EnumProperty, IntoEnumIterator};
use std::str::FromStr;

use crate::language::{prelude::*};

pub use googleapiv1::GoogleApiV1;
pub use googleapiv2::GoogleApiV2;
pub use googlescrape::GoogleScrape;


#[derive(strum_macros::EnumProperty, strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
pub enum Language {
   #[strum(props(code = "auto"), serialize = "auto", serialize = "Automatic")]
    Automatic,
    #[strum(props(code = "af"), serialize = "af", serialize = "Afrikaans")]
    Afrikaans,
    #[strum(props(code = "sq"), serialize = "sq", serialize = "Albanian")]
    Albanian,
    #[strum(props(code = "am"), serialize = "am", serialize = "Amharic")]
    Amharic,
    #[strum(props(code = "ar"), serialize = "ar", serialize = "Arabic")]
    Arabic,
    #[strum(props(code = "hy"), serialize = "hy", serialize = "Armenian")]
    Armenian,
    #[strum(props(code = "as"), serialize = "as", serialize = "Assamese")]
    Assamese,
    #[strum(props(code = "ay"), serialize = "ay", serialize = "Aymara")]
    Aymara,
    #[strum(props(code = "az"), serialize = "az", serialize = "Azerbaijani")]
    Azerbaijani,
    #[strum(props(code = "bm"), serialize = "bm", serialize = "Bambara")]
    Bambara,
    #[strum(props(code = "eu"), serialize = "eu", serialize = "Basque")]
    Basque,
    #[strum(props(code = "be"), serialize = "be", serialize = "Belarusian")]
    Belarusian,
    #[strum(props(code = "bn"), serialize = "bn", serialize = "Bengali")]
    Bengali,
    #[strum(props(code = "bho"), serialize = "bho", serialize = "Bhojpuri")]
    Bhojpuri,
    #[strum(props(code = "bs"), serialize = "bs", serialize = "Bosnian")]
    Bosnian,
    #[strum(props(code = "bg"), serialize = "bg", serialize = "Bulgarian")]
    Bulgarian,
    #[strum(props(code = "ca"), serialize = "ca", serialize = "Catalan")]
    Catalan,
    #[strum(props(code = "ceb"), serialize = "ceb", serialize = "Cebuano")]
    Cebuano,
    #[strum(props(code = "zh"), serialize = "zh", serialize = "ChineseSimplified")]
    ChineseSimplified,
    #[strum(props(code = "zh-TW"), serialize = "zh-TW", serialize = "ChineseTraditional")]
    ChineseTraditional,
    #[strum(props(code = "co"), serialize = "co", serialize = "Corsican")]
    Corsican,
    #[strum(props(code = "hr"), serialize = "hr", serialize = "Croatian")]
    Croatian,
    #[strum(props(code = "cs"), serialize = "cs", serialize = "Czech")]
    Czech,
    #[strum(props(code = "da"), serialize = "da", serialize = "Danish")]
    Danish,
    #[strum(props(code = "dv"), serialize = "dv", serialize = "Dhivehi")]
    Dhivehi,
    #[strum(props(code = "doi"), serialize = "doi", serialize = "Dogri")]
    Dogri,
    #[strum(props(code = "nl"), serialize = "nl", serialize = "Dutch")]
    Dutch,
    #[strum(props(code = "en"), serialize = "en", serialize = "English")]
    English,
    #[strum(props(code = "eo"), serialize = "eo", serialize = "Esperanto")]
    Esperanto,
    #[strum(props(code = "et"), serialize = "et", serialize = "Estonian")]
    Estonian,
    #[strum(props(code = "ee"), serialize = "ee", serialize = "Ewe")]
    Ewe,
    #[strum(props(code = "fil"), serialize = "fil", serialize = "Filipino")]
    Filipino,
    #[strum(props(code = "fi"), serialize = "fi", serialize = "Finnish")]
    Finnish,
    #[strum(props(code = "fr"), serialize = "fr", serialize = "French")]
    French,
    #[strum(props(code = "fy"), serialize = "fy", serialize = "Frisian")]
    Frisian,
    #[strum(props(code = "gl"), serialize = "gl", serialize = "Galician")]
    Galician,
    #[strum(props(code = "ka"), serialize = "ka", serialize = "Georgian")]
    Georgian,
    #[strum(props(code = "de"), serialize = "de", serialize = "German")]
    German,
    #[strum(props(code = "el"), serialize = "el", serialize = "Greek")]
    Greek,
    #[strum(props(code = "gn"), serialize = "gn", serialize = "Guarani")]
    Guarani,
    #[strum(props(code = "gu"), serialize = "gu", serialize = "Gujarati")]
    Gujarati,
    #[strum(props(code = "ht"), serialize = "ht", serialize = "Haitian Creole")]
    HaitianCreole,
    #[strum(props(code = "ha"), serialize = "ha", serialize = "Hausa")]
    Hausa,
    #[strum(props(code = "haw"), serialize = "haw", serialize = "Hawaiian")]
    Hawaiian,
    #[strum(props(code = "he or iw"), serialize = "he or iw", serialize = "Hebrew")]
    Hebrew,
    #[strum(props(code = "hi"), serialize = "hi", serialize = "Hindi")]
    Hindi,
    #[strum(props(code = "hmn"), serialize = "hmn", serialize = "Hmong")]
    Hmong,
    #[strum(props(code = "hu"), serialize = "hu", serialize = "Hungarian")]
    Hungarian,
    #[strum(props(code = "is"), serialize = "is", serialize = "Icelandic")]
    Icelandic,
    #[strum(props(code = "ig"), serialize = "ig", serialize = "Igbo")]
    Igbo,
    #[strum(props(code = "ilo"), serialize = "ilo", serialize = "Ilocano")]
    Ilocano,
    #[strum(props(code = "id"), serialize = "id", serialize = "Indonesian")]
    Indonesian,
    #[strum(props(code = "ga"), serialize = "ga", serialize = "Irish")]
    Irish,
    #[strum(props(code = "it"), serialize = "it", serialize = "Italian")]
    Italian,
    #[strum(props(code = "ja"), serialize = "ja", serialize = "Japanese")]
    Japanese,
    #[strum(props(code = "jv"), serialize = "jv", serialize = "Javanese")]
    Javanese,
    #[strum(props(code = "kn"), serialize = "kn", serialize = "Kannada")]
    Kannada,
    #[strum(props(code = "kk"), serialize = "kk", serialize = "Kazakh")]
    Kazakh,
    #[strum(props(code = "km"), serialize = "km", serialize = "Khmer")]
    Khmer,
    #[strum(props(code = "rw"), serialize = "rw", serialize = "Kinyarwanda")]
    Kinyarwanda,
    #[strum(props(code = "gom"), serialize = "gom", serialize = "Konkani")]
    Konkani,
    #[strum(props(code = "ko"), serialize = "ko", serialize = "Korean")]
    Korean,
    #[strum(props(code = "kri"), serialize = "kri", serialize = "Krio")]
    Krio,
    #[strum(props(code = "ckb"), serialize = "ckb", serialize = "Kurdish")]
    Kurdish,
    #[strum(props(code = "ky"), serialize = "ky", serialize = "Kyrgyz")]
    Kyrgyz,
    #[strum(props(code = "lo"), serialize = "lo", serialize = "Lao")]
    Lao,
    #[strum(props(code = "la"), serialize = "la", serialize = "Latin")]
    Latin,
    #[strum(props(code = "lv"), serialize = "lv", serialize = "Latvian")]
    Latvian,
    #[strum(props(code = "ln"), serialize = "ln", serialize = "Lingala")]
    Lingala,
    #[strum(props(code = "lt"), serialize = "lt", serialize = "Lithuanian")]
    Lithuanian,
    #[strum(props(code = "lg"), serialize = "lg", serialize = "Luganda")]
    Luganda,
    #[strum(props(code = "lb"), serialize = "lb", serialize = "Luxembourgish")]
    Luxembourgish,
    #[strum(props(code = "mk"), serialize = "mk", serialize = "Macedonian")]
    Macedonian,
    #[strum(props(code = "mai"), serialize = "mai", serialize = "Maithili")]
    Maithili,
    #[strum(props(code = "mg"), serialize = "mg", serialize = "Malagasy")]
    Malagasy,
    #[strum(props(code = "ms"), serialize = "ms", serialize = "Malay")]
    Malay,
    #[strum(props(code = "ml"), serialize = "ml", serialize = "Malayalam")]
    Malayalam,
    #[strum(props(code = "mt"), serialize = "mt", serialize = "Maltese")]
    Maltese,
    #[strum(props(code = "mi"), serialize = "mi", serialize = "Maori")]
    Maori,
    #[strum(props(code = "mr"), serialize = "mr", serialize = "Marathi")]
    Marathi,
    #[strum(props(code = "mni-Mtei"), serialize = "mni-Mtei", serialize = "Meiteilon")]
    Meiteilon,
    #[strum(props(code = "lus"), serialize = "lus", serialize = "Mizo")]
    Mizo,
    #[strum(props(code = "mn"), serialize = "mn", serialize = "Mongolian")]
    Mongolian,
    #[strum(props(code = "my"), serialize = "my", serialize = "Myanmar")]
    Myanmar,
    #[strum(props(code = "ne"), serialize = "ne", serialize = "Nepali")]
    Nepali,
    #[strum(props(code = "no"), serialize = "no", serialize = "Norwegian")]
    Norwegian,
    #[strum(props(code = "ny"), serialize = "ny", serialize = "Nyanja")]
    Nyanja,
    #[strum(props(code = "or"), serialize = "or", serialize = "Odia")]
    Odia,
    #[strum(props(code = "om"), serialize = "om", serialize = "Oromo")]
    Oromo,
    #[strum(props(code = "ps"), serialize = "ps", serialize = "Pashto")]
    Pashto,
    #[strum(props(code = "fa"), serialize = "fa", serialize = "Persian")]
    Persian,
    #[strum(props(code = "pl"), serialize = "pl", serialize = "Polish")]
    Polish,
    #[strum(props(code = "pt"), serialize = "pt", serialize = "Portuguese")]
    Portuguese,
    #[strum(props(code = "pa"), serialize = "pa", serialize = "Punjabi")]
    Punjabi,
    #[strum(props(code = "qu"), serialize = "qu", serialize = "Quechua")]
    Quechua,
    #[strum(props(code = "ro"), serialize = "ro", serialize = "Romanian")]
    Romanian,
    #[strum(props(code = "ru"), serialize = "ru", serialize = "Russian")]
    Russian,
    #[strum(props(code = "sm"), serialize = "sm", serialize = "Samoan")]
    Samoan,
    #[strum(props(code = "sa"), serialize = "sa", serialize = "Sanskrit")]
    Sanskrit,
    #[strum(props(code = "gd"), serialize = "gd", serialize = "Scots Gaelic")]
    ScotsGaelic,
    #[strum(props(code = "nso"), serialize = "nso", serialize = "Sepedi")]
    Sepedi,
    #[strum(props(code = "sr"), serialize = "sr", serialize = "Serbian")]
    Serbian,
    #[strum(props(code = "st"), serialize = "st", serialize = "Sesotho")]
    Sesotho,
    #[strum(props(code = "sn"), serialize = "sn", serialize = "Shona")]
    Shona,
    #[strum(props(code = "sd"), serialize = "sd", serialize = "Sindhi")]
    Sindhi,
    #[strum(props(code = "si"), serialize = "si", serialize = "Sinhalese")]
    Sinhalese,
    #[strum(props(code = "sk"), serialize = "sk", serialize = "Slovak")]
    Slovak,
    #[strum(props(code = "sl"), serialize = "sl", serialize = "Slovenian")]
    Slovenian,
    #[strum(props(code = "so"), serialize = "so", serialize = "Somali")]
    Somali,
    #[strum(props(code = "es"), serialize = "es", serialize = "Spanish")]
    Spanish,
    #[strum(props(code = "su"), serialize = "su", serialize = "Sundanese")]
    Sundanese,
    #[strum(props(code = "sw"), serialize = "sw", serialize = "Swahili")]
    Swahili,
    #[strum(props(code = "sv"), serialize = "sv", serialize = "Swedish")]
    Swedish,
    #[strum(props(code = "tl"), serialize = "tl", serialize = "Tagalog")]
    Tagalog,
    #[strum(props(code = "tg"), serialize = "tg", serialize = "Tajik")]
    Tajik,
    #[strum(props(code = "ta"), serialize = "ta", serialize = "Tamil")]
    Tamil,
    #[strum(props(code = "tt"), serialize = "tt", serialize = "Tatar")]
    Tatar,
    #[strum(props(code = "te"), serialize = "te", serialize = "Telugu")]
    Telugu,
    #[strum(props(code = "th"), serialize = "th", serialize = "Thai")]
    Thai,
    #[strum(props(code = "ti"), serialize = "ti", serialize = "Tigrinya")]
    Tigrinya,
    #[strum(props(code = "ts"), serialize = "ts", serialize = "Tsonga")]
    Tsonga,
    #[strum(props(code = "tr"), serialize = "tr", serialize = "Turkish")]
    Turkish,
    #[strum(props(code = "tk"), serialize = "tk", serialize = "Turkmen")]
    Turkmen,
    #[strum(props(code = "ak"), serialize = "ak", serialize = "Akan")]
    Akan,
    #[strum(props(code = "uk"), serialize = "uk", serialize = "Ukrainian")]
    Ukrainian,
    #[strum(props(code = "ur"), serialize = "ur", serialize = "Urdu")]
    Urdu,
    #[strum(props(code = "ug"), serialize = "ug", serialize = "Uyghur")]
    Uyghur,
    #[strum(props(code = "uz"), serialize = "uz", serialize = "Uzbek")]
    Uzbek,
    #[strum(props(code = "vi"), serialize = "vi", serialize = "Vietnamese")]
    Vietnamese,
    #[strum(props(code = "cy"), serialize = "cy", serialize = "Welsh")]
    Welsh,
    #[strum(props(code = "xh"), serialize = "xh", serialize = "Xhosa")]
    Xhosa,
    #[strum(props(code = "yi"), serialize = "yi", serialize = "Yiddish")]
    Yiddish,
    #[strum(props(code = "yo"), serialize = "yo", serialize = "Yoruba")]
    Yoruba,
    #[strum(props(code = "zu"), serialize = "zu", serialize = "Zulu")]
    Zulu
}


impl LanguageExt for Language {
    fn from_language_code(code: &str) -> Option<Box<Self>> {
      match Language::from_str(code) {
         Ok(lang) => {
            Some(Box::new(lang))
         },
         Err(..) => { None }
      }
    }

    fn from_language_name(name: &str) -> Option<Box<Self>> {
      Language::from_language_code(name)
    }
 
    fn to_language_code(&self) -> String {
      self.get_str("code").unwrap().to_string()
    }

    fn to_language_name(&self) -> String {
      self.to_string()
    }

    fn get_iterator() -> Box<dyn Iterator<Item = Self>> {
        Box::new(Language::iter())
    }
}
