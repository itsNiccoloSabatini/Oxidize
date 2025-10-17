use egui::Color32;

#[non_exhaustive]
pub enum OxidizeColor {
    IOSBlue,
    #[expect(dead_code)] // Not currently used
    DarkAlternativeBlue,
    #[expect(dead_code)] // Not currently used
    SystemGreen,
    #[expect(dead_code)] // Not currently used
    SystemRed,
    #[expect(dead_code)] // Not currently used
    ModernPurple,
}

impl OxidizeColor {
    pub fn to_egui_c32(&self) -> Color32 {
        match self {
            Self::IOSBlue => Color32::from_rgb(64, 149, 255),
            Self::DarkAlternativeBlue => Color32::from_rgb(0, 122, 255),
            Self::SystemGreen => Color32::from_rgb(52, 199, 89),
            Self::SystemRed => Color32::from_rgb(255, 59, 48),
            Self::ModernPurple => Color32::from_rgb(175, 82, 222),
        }
    }
}
