use crate::{app::sidepanel::buttons::draw_side_panel_buttons, OxidiseApp};

pub fn draw_side_panel_ui(
    ctx: &egui::Context,
    ox_app: &mut OxidiseApp,
) {
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Oxidise");
            ui.separator();

            // Draw the main panel toggle button
            draw_side_panel_buttons(ui, ox_app);
        });
}