use egui::{Color32, RichText};

use crate::{OxidizeApp, app::mainpanel::OxidizeMainpanel};

pub fn draw_side_panel_buttons(ui: &mut egui::Ui, ox_app: &mut OxidizeApp) {
    let dashboard_button = draw_dashboard_button(ui, &ox_app.mainpanel);
    let settings_button = draw_settings_button(ui, &ox_app.mainpanel);
    if dashboard_button.clicked() {
        ox_app.mainpanel.set_dashboard();
    }
    if settings_button.clicked() {
        ox_app.mainpanel.set_settings();
    }
}

fn draw_dashboard_button(ui: &mut egui::Ui, active_menu: &OxidizeMainpanel) -> egui::Response {
    let default_text_color = ui.style().visuals.text_color();
    let text =
        RichText::new("🏠 Home")
            .size(32.0)
            .color(if *active_menu == OxidizeMainpanel::Dashboard {
                Color32::from_rgb(64, 149, 255) // iOS Blue - much more visible
            // Color32::from_rgb(0, 122, 255)   // Dark Blue - alternative
            // Color32::from_rgb(52, 199, 89)   // System Green
            // Color32::from_rgb(255, 59, 48)   // System Red
            // Color32::from_rgb(175, 82, 222)  // Modern Purple
            } else {
                default_text_color
            });
    ui.add(egui::Button::new(text).min_size(egui::vec2(200.0, 0.0)))
}
fn draw_settings_button(ui: &mut egui::Ui, active_menu: &OxidizeMainpanel) -> egui::Response {
    let default_text_color = ui.style().visuals.text_color();
    let text = RichText::new("⚙ Settings").size(32.0).color(
        if *active_menu == OxidizeMainpanel::Settings {
            Color32::from_rgb(64, 149, 255) // iOS Blue - much more visible
        // Color32::from_rgb(0, 122, 255)   // Dark Blue - alternative
        // Color32::from_rgb(52, 199, 89)   // System Green
        // Color32::from_rgb(255, 59, 48)   // System Red
        // Color32::from_rgb(175, 82, 222)  // Modern Purple
        } else {
            default_text_color
        },
    );
    ui.add(egui::Button::new(text).min_size(egui::vec2(200.0, 0.0)))
}
