use crate::{OxidiseApp, app::size::Size};

mod dashboard;
mod settings;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum OxidiseMainpanel {
    #[default]
    Dashboard,
    Settings,
}

impl OxidiseMainpanel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Settings => "Settings",
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Self::Dashboard => Self::Settings,
            Self::Settings => Self::Dashboard,
        };
    }
}

pub fn draw_mainpanel(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    ox_app: &OxidiseApp,
) {
    match ox_app.mainpanel {
        OxidiseMainpanel::Dashboard => dashboard::draw_dashboard(ctx, ui, frame, &ox_app.sizes),
        OxidiseMainpanel::Settings => settings::draw_settings(ctx, ui, frame, &ox_app.sizes),
    }
}

pub fn draw_mainpanel_heading(str: &str, ui: &mut egui::Ui, sizes: &Size) {
    ui.label(egui::RichText::new(str).size(sizes.text_heading_size()));
    ui.separator();
    ui.add_space(sizes.space_after_heading());
}
