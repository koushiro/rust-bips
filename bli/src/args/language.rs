use bip0039::BuiltInLanguage;
use clap::ValueEnum;
use serde::Serialize;

#[derive(Clone, Copy, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
    Czech,
    French,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Spanish,
}

impl Language {
    pub fn name(self) -> String {
        self.to_possible_value()
            .expect("all languages have a name")
            .get_name()
            .to_owned()
    }
}

impl From<Language> for BuiltInLanguage {
    fn from(language: Language) -> Self {
        match language {
            Language::English => Self::English,
            Language::ChineseSimplified => Self::ChineseSimplified,
            Language::ChineseTraditional => Self::ChineseTraditional,
            Language::Czech => Self::Czech,
            Language::French => Self::French,
            Language::Italian => Self::Italian,
            Language::Japanese => Self::Japanese,
            Language::Korean => Self::Korean,
            Language::Portuguese => Self::Portuguese,
            Language::Spanish => Self::Spanish,
        }
    }
}
