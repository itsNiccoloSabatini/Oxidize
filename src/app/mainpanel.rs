use crate::{OxidizeApp, app::size::Size};
use rust_i18n::t;

mod dashboard;
mod settings;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxidizeMainpanel {
    #[default]
    Dashboard,
    Settings,
}

impl OxidizeMainpanel {
    pub fn as_str(&self) -> String {
        match self {
            Self::Dashboard => t!("Dashboard").to_string(),
            Self::Settings => t!("Settings").to_string(),
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Self::Dashboard => Self::Settings,
            Self::Settings => Self::Dashboard,
        };
    }

    pub fn set_dashboard(&mut self) {
        *self = Self::Dashboard;
    }

    pub fn set_settings(&mut self) {
        *self = Self::Settings;
    }
}

pub fn draw_mainpanel(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    ox_app: &mut OxidizeApp,
) {
    match ox_app.mainpanel {
        OxidizeMainpanel::Dashboard => dashboard::draw_dashboard(ctx, ui, frame, &ox_app.sizes),
        OxidizeMainpanel::Settings => {
            settings::draw_settings(
                ctx,
                ui,
                frame,
                &ox_app.sizes,
                &mut ox_app.language,
                &mut ox_app.color_theme,
            );
        }
    }
}

pub fn draw_mainpanel_heading(str: &str, ui: &mut egui::Ui, sizes: &Size) {
    ui.label(egui::RichText::new(str).size(sizes.text_heading_size()));
    ui.separator();
    ui.add_space(sizes.space_after_heading());
}
