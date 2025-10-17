use egui::Color32;
use rust_i18n::t;

#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OxidizeThemeColor {
    #[default]
    IOSBlue,
    DarkAlternativeBlue,
    SystemGreen,
    SystemRed,
    ModernPurple,
}

impl OxidizeThemeColor {
    pub fn as_string(&self) -> String {
        match self {
            Self::IOSBlue => t!("ios-blue").to_string(),
            Self::DarkAlternativeBlue => t!("dark-alt-blue").to_string(),
            Self::SystemGreen => t!("system-green").to_string(),
            Self::SystemRed => t!("system-red").to_string(),
            Self::ModernPurple => t!("modern-purple").to_string(),
        }
    }
    pub fn as_egui_c32(&self) -> Color32 {
        match self {
            Self::IOSBlue => Color32::from_rgb(64, 149, 255),
            Self::DarkAlternativeBlue => Color32::from_rgb(0, 122, 255),
            Self::SystemGreen => Color32::from_rgb(52, 199, 89),
            Self::SystemRed => Color32::from_rgb(255, 59, 48),
            Self::ModernPurple => Color32::from_rgb(175, 82, 222),
        }
    }
}
