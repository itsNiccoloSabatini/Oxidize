use crate::rust_i18n::t;
use crate::{
    Language,
    app::{color::OxidizeThemeColor, mainpanel::draw_mainpanel_heading, size::Size},
};
use egui::{RichText, Theme, ThemePreference};

pub fn draw_settings(
    _ctx: &egui::Context,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    sizes: &Size,
    language: &mut Language,
    color_theme: &mut OxidizeThemeColor,
) {
    draw_mainpanel_heading(&t!("settings"), ui, sizes);
    ui.horizontal(|ui| {
        let theme_text = RichText::new(t!("theme")).size(sizes.text_body_size());
        ui.label(theme_text);
        global_theme_preference_buttons(ui, sizes);
    });
    ui.horizontal(|ui| {
        ui.label(RichText::new(t!("select-color")).size(sizes.text_body_size()));
        egui::ComboBox::from_id_salt("color-choice")
            .selected_text(color_theme.as_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(color_theme, OxidizeThemeColor::IOSBlue, t!("ios-blue"));
                ui.selectable_value(
                    color_theme,
                    OxidizeThemeColor::DarkAlternativeBlue,
                    t!("dark-alt-blue"),
                );
                ui.selectable_value(
                    color_theme,
                    OxidizeThemeColor::SystemGreen,
                    t!("system-green"),
                );
                ui.selectable_value(color_theme, OxidizeThemeColor::SystemRed, t!("system-red"));
                ui.selectable_value(
                    color_theme,
                    OxidizeThemeColor::ModernPurple,
                    t!("modern-purple"),
                );
            });
    });
    ui.horizontal(|ui| {
        ui.label(RichText::new(t!("lang")).size(sizes.text_body_size()));
        egui::ComboBox::from_id_salt("lang-choice")
            .selected_text(language.as_str())
            .show_ui(ui, |ui| {
                ui.selectable_value(language, Language::ItIt, "Italiano");
                ui.selectable_value(language, Language::EnUs, "English");
            });
    });
}

fn global_theme_preference_buttons(ui: &mut egui::Ui, sizes: &Size) {
    let mut theme_preference = ui.ctx().options(|opt| opt.theme_preference);
    theme_preference_buttons(ui, &mut theme_preference, sizes);
    ui.ctx().set_theme(theme_preference);
}

fn theme_preference_buttons(
    ui: &mut egui::Ui,
    theme_preference: &mut ThemePreference,
    sizes: &Size,
) {
    ui.horizontal(|ui| {
        let system_theme = ui.ctx().input(|i| i.raw.system_theme);
        let system_text = RichText::new(t!("system")).size(sizes.text_body_size());
        ui.selectable_value(theme_preference, ThemePreference::System, system_text)
            .on_hover_ui(|ui| {
                ui.label("Follow the system theme preference.");

                ui.add_space(4.0);

                if let Some(system_theme) = system_theme {
                    ui.label(format!(
                        "The current system theme is: {}",
                        match system_theme {
                            Theme::Dark => "dark",
                            Theme::Light => "light",
                        }
                    ));
                } else {
                    ui.label("The system theme is unknown.");
                }
            });
        let dark_text = RichText::new(t!("dark")).size(sizes.text_body_size());
        ui.selectable_value(theme_preference, ThemePreference::Dark, dark_text)
            .on_hover_text("Use the dark mode theme");

        let light_text = RichText::new(t!("light")).size(sizes.text_body_size());
        ui.selectable_value(theme_preference, ThemePreference::Light, light_text)
            .on_hover_text("Use the light mode theme");
    });
}
