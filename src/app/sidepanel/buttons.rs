use egui::RichText;

use crate::OxidiseApp;

pub fn draw_side_panel_buttons(ui: &mut egui::Ui, ox_app: &mut OxidiseApp) {
    let dashboard_button = draw_dashboard_button(ui);
    let settings_button = draw_settings_button(ui);
    if dashboard_button.clicked() {
        ox_app.mainpanel.set_dashboard();
    }
    if settings_button.clicked() {
        ox_app.mainpanel.set_settings();
    }
}

fn draw_dashboard_button(ui: &mut egui::Ui) -> egui::Response {
    let text = RichText::new("🏠 Home").size(32.0);
    ui.add(egui::Button::new(text))
}
fn draw_settings_button(ui: &mut egui::Ui) -> egui::Response {
    let text = RichText::new("⚙ Settings").size(32.0);
    ui.add(egui::Button::new(text))
}
