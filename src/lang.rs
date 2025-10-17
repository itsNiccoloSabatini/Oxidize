#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Language {
    #[default]
    ItIt,
    EnUs,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ItIt => "Italiano",
            Self::EnUs => "English",
        }
    }

    pub fn as_i18n_code(&self) -> &str {
        match self {
            Self::ItIt => "it",
            Self::EnUs => "en",
        }
    }
}
