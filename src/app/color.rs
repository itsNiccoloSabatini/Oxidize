use egui::Color32;

#[non_exhaustive]
pub enum OxidizeColor {
    IOSBlue,
    DarkAlternativeBlue,
    SystemGreen,
    SystemRed,
    ModernPurple,
}

impl OxidizeColor {
    pub fn to_egui_c32(&self) -> Color32 {
        match self {
            OxidizeColor::IOSBlue => Color32::from_rgb(64, 149, 255),
            OxidizeColor::DarkAlternativeBlue => Color32::from_rgb(0, 122, 255),
            OxidizeColor::SystemGreen => Color32::from_rgb(52, 199, 89),
            OxidizeColor::SystemRed => Color32::from_rgb(255, 59, 48),
            OxidizeColor::ModernPurple => Color32::from_rgb(175, 82, 222),
        }
    }
}