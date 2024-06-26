pub mod googleapiv1;
pub mod googleapiv2;
pub mod googlescrape;

use std::collections::BTreeMap;

pub use googleapiv1::GoogleApiV1;
pub use googleapiv2::GoogleApiV2;
pub use googlescrape::GoogleScrape;

fn get_language_map() -> BTreeMap<&'static str, &'static str> {
    let mut map = BTreeMap::new();
    map.insert("Automatic", "auto");
    map.insert("Afrikaans", "af");
    map.insert("Albanian", "sq");
    map.insert("Amharic", "am");
    map.insert("Arabic", "ar");
    map.insert("Armenian", "hy");
    map.insert("Assamese", "as");
    map.insert("Aymara", "ay");
    map.insert("Azerbaijani", "az");
    map.insert("Bambara", "bm");
    map.insert("Basque", "eu");
    map.insert("Belarusian", "be");
    map.insert("Bengali", "bn");
    map.insert("Bhojpuri", "bho");
    map.insert("Bosnian", "bs");
    map.insert("Bulgarian", "bg");
    map.insert("Catalan", "ca");
    map.insert("Cebuano", "ceb");
    map.insert("ChineseSimplified", "zh");
    map.insert("ChineseTraditional", "zh-TW");
    map.insert("Corsican", "co");
    map.insert("Croatian", "hr");
    map.insert("Czech", "cs");
    map.insert("Danish", "da");
    map.insert("Dhivehi", "dv");
    map.insert("Dogri", "doi");
    map.insert("Dutch", "nl");
    map.insert("English", "en");
    map.insert("Esperanto", "eo");
    map.insert("Estonian", "et");
    map.insert("Ewe", "ee");
    map.insert("Filipino", "fil");
    map.insert("Finnish", "fi");
    map.insert("French", "fr");
    map.insert("Frisian", "fy");
    map.insert("Galician", "gl");
    map.insert("Georgian", "ka");
    map.insert("German", "de");
    map.insert("Greek", "el");
    map.insert("Guarani", "gn");
    map.insert("Gujarati", "gu");
    map.insert("Haitian Creole", "ht");
    map.insert("Hausa", "ha");
    map.insert("Hawaiian", "haw");
    map.insert("Hebrew", "he or iw");
    map.insert("Hindi", "hi");
    map.insert("Hmong", "hmn");
    map.insert("Hungarian", "hu");
    map.insert("Icelandic", "is");
    map.insert("Igbo", "ig");
    map.insert("Ilocano", "ilo");
    map.insert("Indonesian", "id");
    map.insert("Irish", "ga");
    map.insert("Italian", "it");
    map.insert("Japanese", "ja");
    map.insert("Javanese", "jv");
    map.insert("Kannada", "kn");
    map.insert("Kazakh", "kk");
    map.insert("Khmer", "km");
    map.insert("Kinyarwanda", "rw");
    map.insert("Konkani", "gom");
    map.insert("Korean", "ko");
    map.insert("Krio", "kri");
    map.insert("Kurdish", "ckb");
    map.insert("Kyrgyz", "ky");
    map.insert("Lao", "lo");
    map.insert("Latin", "la");
    map.insert("Latvian", "lv");
    map.insert("Lingala", "ln");
    map.insert("Lithuanian", "lt");
    map.insert("Luganda", "lg");
    map.insert("Luxembourgish", "lb");
    map.insert("Macedonian", "mk");
    map.insert("Maithili", "mai");
    map.insert("Malagasy", "mg");
    map.insert("Malay", "ms");
    map.insert("Malayalam", "ml");
    map.insert("Maltese", "mt");
    map.insert("Maori", "mi");
    map.insert("Marathi", "mr");
    map.insert("Meiteilon", "mni-Mtei");
    map.insert("Mizo", "lus");
    map.insert("Mongolian", "mn");
    map.insert("Myanmar", "my");
    map.insert("Nepali", "ne");
    map.insert("Norwegian", "no");
    map.insert("Nyanja", "ny");
    map.insert("Odia", "or");
    map.insert("Oromo", "om");
    map.insert("Pashto", "ps");
    map.insert("Persian", "fa");
    map.insert("Polish", "pl");
    map.insert("Portuguese", "pt");
    map.insert("Punjabi", "pa");
    map.insert("Quechua", "qu");
    map.insert("Romanian", "ro");
    map.insert("Russian", "ru");
    map.insert("Samoan", "sm");
    map.insert("Sanskrit", "sa");
    map.insert("Scots Gaelic", "gd");
    map.insert("Sepedi", "nso");
    map.insert("Serbian", "sr");
    map.insert("Sesotho", "st");
    map.insert("Shona", "sn");
    map.insert("Sindhi", "sd");
    map.insert("Sinhalese", "si");
    map.insert("Slovak", "sk");
    map.insert("Slovenian", "sl");
    map.insert("Somali", "so");
    map.insert("Spanish", "es");
    map.insert("Sundanese", "su");
    map.insert("Swahili", "sw");
    map.insert("Swedish", "sv");
    map.insert("Tagalog", "tl");
    map.insert("Tajik", "tg");
    map.insert("Tamil", "ta");
    map.insert("Tatar", "tt");
    map.insert("Telugu", "te");
    map.insert("Thai", "th");
    map.insert("Tigrinya", "ti");
    map.insert("Tsonga", "ts");
    map.insert("Turkish", "tr");
    map.insert("Turkmen", "tk");
    map.insert("Akan", "ak");
    map.insert("Ukrainian", "uk");
    map.insert("Urdu", "ur");
    map.insert("Uyghur", "ug");
    map.insert("Uzbek", "uz");
    map.insert("Vietnamese", "vi");
    map.insert("Welsh", "cy");
    map.insert("Xhosa", "xh");
    map.insert("Yiddish", "yi");
    map.insert("Yoruba", "yo");
    map.insert("Zulu", "zu");

    map
}
