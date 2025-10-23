use egui::{RichText, style::Selection};

use crate::{OxidizeApp, app::sidepanel::buttons::draw_side_panel_buttons};

pub fn draw_side_panel_ui(ctx: &egui::Context, ox_app: &mut OxidizeApp) {
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .min_width(250.0)
        .show(ctx, |ui| {
            let mut visuals = ui.visuals().clone();
            visuals.selection = Selection {
                bg_fill: ox_app.color_theme.as_egui_c32_selection(),
                stroke: ox_app.color_theme.as_egui_stroke(),
            };
            ctx.set_visuals(visuals);
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
