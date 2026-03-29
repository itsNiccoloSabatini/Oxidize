mod color;
mod mainpanel;
mod sidepanel;
mod size;

use egui::style::{Selection, WidgetVisuals};
use egui::{CornerRadius, Stroke};

use crate::Language;
use crate::app::color::OxidizeThemeColor;
use crate::app::mainpanel::OxidizeMainpanel;
use crate::app::sidepanel::draw_side_panel_ui;
use crate::app::size::Size;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct OxidizeApp {
    /// The app state is stored in a single struct, which can be serialized and deserialized.
    /// You can add more fields to this struct as needed.

    /// The current main panel being displayed.
    #[serde(skip)]
    pub mainpanel: OxidizeMainpanel,
    /// The sizes used for various UI elements.
    /// This is used to ensure consistent sizing across the application.
    #[serde(skip)]
    pub sizes: Size,
    /// The current language of the application.
    /// This is used for internationalization.
    pub language: Language,
    /// The current color theme of the application.
    /// This is used to ensure consistent theming across the application.
    pub color_theme: OxidizeThemeColor,
}

impl OxidizeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn active_open_widget_visuals(&self) -> WidgetVisuals {
        WidgetVisuals {
            bg_fill: self.color_theme.as_egui_c32(),
            weak_bg_fill: self.color_theme.as_egui_c32(),
            bg_stroke: Stroke::NONE,
            corner_radius: CornerRadius::same(2),
            fg_stroke: self.color_theme.as_egui_stroke(),
            expansion: 0.0,
        }
    }

    pub fn hovered_widget_visuals(&self) -> WidgetVisuals {
        WidgetVisuals {
            bg_fill: self.color_theme.as_egui_c32_hover(),
            weak_bg_fill: self.color_theme.as_egui_c32_hover(),
            bg_stroke: Stroke::NONE,
            corner_radius: CornerRadius::same(2),
            fg_stroke: self.color_theme.as_egui_stroke(),
            expansion: 0.0,
        }
    }
}

impl eframe::App for OxidizeApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        rust_i18n::set_locale(self.language.as_i18n_code());

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.visuals_mut().selection = Selection {
                bg_fill: self.color_theme.as_egui_c32_selection(),
                stroke: self.color_theme.as_egui_stroke(),
            };
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });
        draw_side_panel_ui(ui, self);
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            mainpanel::draw_mainpanel(ui, _frame, self);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
