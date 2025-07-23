use crate::OxidiseApp;

mod dashboard;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum OxidiseMainpanel {
    #[default]
    Dashboard,
}

pub fn draw_mainpanel(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    ox_app: &mut OxidiseApp,
) {
    ox_app.mainpanel = OxidiseMainpanel::Dashboard; // Set default main panel
    match ox_app.mainpanel {
        OxidiseMainpanel::Dashboard => dashboard::draw_dashboard(ctx, ui, frame),
    }
}

pub fn draw_mainpanel_heading(str: &str, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new(str).size(24.0));
    ui.separator();
}
