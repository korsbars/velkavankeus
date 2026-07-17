use crate::app::App;
use crate::ui::gui::{dialogs, panels};
use eframe::egui;
// FIXED: Point directly to simulation module here as well
use crate::sim::simulation::FinancialComparison; 

pub struct GuiApp {
    pub app: App,

    pub show_add_dialog: bool,
    pub new_name: String,
    pub new_amount: String,
    pub new_interest: String,
    pub new_minimum: String,

    pub show_edit_dialog: bool,
    pub edit_name: String,
    pub edit_amount: String,
    pub edit_interest: String,
    pub edit_minimum: String,

    pub show_save_dialog: bool,
    pub show_load_dialog: bool,
    pub filename: String,
    pub io_status: Option<String>,

    pub selected_loan: Option<u64>,

    pub invest_extra: String,
    pub invest_return_pct: String,
    pub invest_strategy_index: usize,
    pub invest_only_extra: bool,
    pub invest_result: Option<FinancialComparison>,

    pub show_debt_charts: bool,
    pub show_reinvest_charts: bool,
    pub show_flat_charts: bool,
}

impl GuiApp {
    pub fn new() -> Self {
        Self {
            app: App::new(),
            show_add_dialog: false,
            new_name: String::new(),
            new_amount: String::new(),
            new_interest: String::new(),
            new_minimum: String::new(),

            show_edit_dialog: false,
            edit_name: String::new(),
            edit_amount: String::new(),
            edit_interest: String::new(),
            edit_minimum: String::new(),

            show_save_dialog: false,
            show_load_dialog: false,
            filename: String::new(),
            io_status: None,

            selected_loan: None,

            invest_extra: "100".to_string(),
            invest_return_pct: "7".to_string(),
            invest_strategy_index: 0,
	    invest_only_extra: false,
            invest_result: None,

	    show_debt_charts: true,
            show_reinvest_charts: true,
            show_flat_charts: true,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            panels::top_toolbar(ui, self);
        });

        egui::SidePanel::left("portfolio").show(ctx, |ui| {
            panels::portfolio_panel(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            panels::details_panel(ui, self);
            ui.separator();
            panels::invest_panel(ui, self);
            ui.separator();
            panels::charts_panel(ui, self);
        });

        dialogs::add_loan_dialog(ctx, self);
        dialogs::edit_loan_dialog(ctx, self);
        dialogs::save_dialog(ctx, self);
        dialogs::load_dialog(ctx, self);
    }
}