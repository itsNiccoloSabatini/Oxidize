use egui::RichText;

use crate::{OxidizeApp, app::sidepanel::buttons::draw_side_panel_buttons};

pub fn draw_side_panel_ui(ctx: &egui::Context, ox_app: &mut OxidizeApp) {
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .min_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Oxidize");
            ui.separator();

            // Draw the main panel toggle button
            draw_side_panel_buttons(ui, ox_app);

            egui::TopBottomPanel::bottom("bottom_panel").show_inside(ui, |ui| {
                ui.label(
                    RichText::new("© 2024 Oxidize - Niccolò Sabatini")
                        .size(ox_app.sizes.copy_right_text_size()),
                );
            });
        });
}
