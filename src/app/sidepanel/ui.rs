use egui::{RichText, style::Selection};

use crate::{OxidizeApp, app::sidepanel::buttons::draw_side_panel_buttons};

pub fn draw_side_panel_ui(ui: &mut egui::Ui, ox_app: &mut OxidizeApp) {
    egui::Panel::left("side_panel")
        .resizable(false)
        .show_inside(ui, |ui| {
            let mut visuals = ui.visuals().clone();
            visuals.selection = Selection {
                bg_fill: ox_app.color_theme.as_egui_c32_selection(),
                stroke: ox_app.color_theme.as_egui_stroke(),
            };
            ui.set_visuals(visuals);
            ui.heading("Oxidize");
            ui.separator();

            // Draw the main panel toggle button
            draw_side_panel_buttons(ui, ox_app);

            egui::Panel::bottom("bottom_panel").show_inside(ui, |ui| {
                ui.label(
                    RichText::new("© 2024 Oxidize - Niccolò Sabatini")
                        .size(ox_app.sizes.copy_right_text_size()),
                );
            });
        });
}
