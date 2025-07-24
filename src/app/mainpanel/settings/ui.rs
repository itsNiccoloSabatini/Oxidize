use crate::app::{mainpanel::draw_mainpanel_heading, size::Size};
use egui::{RichText, Theme, ThemePreference};

const SETTINGS_TITLE: &str = "⚙ Settings";
pub fn draw_settings(
    _ctx: &egui::Context,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    sizes: &Size,
) {
    draw_mainpanel_heading(SETTINGS_TITLE, ui, sizes);
    ui.horizontal(|ui| {
        let theme_text = RichText::new("Oxidize theme: ").size(sizes.text_body_size());
        ui.label(theme_text);
        global_theme_preference_buttons(ui, sizes);
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
        let system_text = RichText::new("💻 System").size(sizes.text_body_size());
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
        let dark_text = RichText::new("🌙 Dark").size(sizes.text_body_size());
        ui.selectable_value(theme_preference, ThemePreference::Dark, dark_text)
            .on_hover_text("Use the dark mode theme");

        let light_text = RichText::new("☀ Light").size(sizes.text_body_size());
        ui.selectable_value(theme_preference, ThemePreference::Light, light_text)
            .on_hover_text("Use the light mode theme");
    });
}
