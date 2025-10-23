use eframe::glow::{STENCIL_FAIL, STENCIL_FUNC};
use egui::{Color32, Stroke};
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
    
    pub fn as_egui_c32_dark(&self) -> Color32 {
        match self {
            Self::IOSBlue => Color32::from_rgb(0, 46, 102),
            Self::DarkAlternativeBlue => Color32::from_rgb(0, 49, 102),
            Self::SystemGreen => Color32::from_rgb(21, 81, 36),
            Self::SystemRed => Color32::from_rgb(51, 3, 0),
            Self::ModernPurple => Color32::from_rgb(63, 16, 86),
        }
    }
    
    pub fn as_egui_c32_hover(&self) -> Color32 {
        match self {
            Self::IOSBlue => Color32::from_rgb(179, 213, 255),
            Self::DarkAlternativeBlue => Color32::from_rgb(179, 215, 255),
            Self::SystemGreen => Color32::from_rgb(194, 239, 205),
            Self::SystemRed => Color32::from_rgb(255, 184, 179),
            Self::ModernPurple => Color32::from_rgb(226, 190, 243),
        }
    }
    
     pub fn as_egui_c32_selection(&self) -> Color32 {
         match self {
             Self::IOSBlue => Color32::from_rgb(179, 213, 255),
             Self::DarkAlternativeBlue => Color32::from_rgb(179, 215, 255),
             Self::SystemGreen => Color32::from_rgb(194, 239, 205),
             Self::SystemRed => Color32::from_rgb(255, 184, 179),
             Self::ModernPurple => Color32::from_rgb(226, 190, 243),
         }
     }
    
    pub fn as_egui_stroke(&self) -> Stroke {
        match self {
            Self::IOSBlue => Stroke::new(2.0, Self::IOSBlue.as_egui_c32_dark()),
            Self::DarkAlternativeBlue => Stroke::new(2.0, Self::DarkAlternativeBlue.as_egui_c32_dark()),
            Self::SystemGreen => Stroke::new(2.0, Self::SystemGreen.as_egui_c32_dark()),
            Self::SystemRed => Stroke::new(2.0, Self::SystemRed.as_egui_c32_dark()),
            Self::ModernPurple => Stroke::new(2.0, Self::ModernPurple.as_egui_c32_dark()),
        }
    }
}
