use crate::app::mainpanel::draw_mainpanel_heading;

const DASHBOARD_TITLE: &str = "Dashboard";
pub fn draw_dashboard(_ctx: &egui::Context, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    draw_mainpanel_heading(DASHBOARD_TITLE, ui);
}
