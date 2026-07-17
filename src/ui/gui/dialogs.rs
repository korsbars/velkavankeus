use crate::ui::gui::window::GuiApp;
use crate::modeling::loan::Loan;
use crate::storage::json;

use eframe::egui;

pub fn add_loan_dialog(ctx: &egui::Context, gui: &mut GuiApp) {
    if !gui.show_add_dialog {
        return;
    }

    egui::Window::new("Add Loan")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut gui.new_name);

            ui.separator();

            ui.label("Amount");
            ui.text_edit_singleline(&mut gui.new_amount);

            ui.label("Interest (%)");
            ui.text_edit_singleline(&mut gui.new_interest);

            ui.label("Minimum Payment");
            ui.text_edit_singleline(&mut gui.new_minimum);

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    gui.show_add_dialog = false;
                }

                if ui.button("Add Loan").clicked() {
                    if let (Ok(amount), Ok(rate), Ok(minimum)) = (
                        gui.new_amount.parse::<f64>(),
                        gui.new_interest.parse::<f64>(),
                        gui.new_minimum.parse::<f64>(),
                    ) {
                        let id = gui.app.portfolio().loan_count() as u64 + 1;

                        gui.app.portfolio_mut().add_loan(Loan::new(
                            id,
                            gui.new_name.clone(),
                            amount,
                            rate,
                            minimum,
                        ));

                        gui.new_name.clear();
                        gui.new_amount.clear();
                        gui.new_interest.clear();
                        gui.new_minimum.clear();
                        gui.show_add_dialog = false;
                    }
                }
            });
        });
}

pub fn open_edit_dialog(gui: &mut GuiApp) {
    let Some(id) = gui.selected_loan else { return };
    let Some(loan) = gui.app.portfolio().loans().iter().find(|l| l.id() == id) else {
        return;
    };

    gui.edit_name = loan.name().to_string();
    gui.edit_amount = loan.original_amount().to_string();
    gui.edit_interest = loan.interest_rate().to_string();
    gui.edit_minimum = loan.minimum_payment().to_string();
    gui.show_edit_dialog = true;
}

pub fn edit_loan_dialog(ctx: &egui::Context, gui: &mut GuiApp) {
    if !gui.show_edit_dialog {
        return;
    }

    let Some(id) = gui.selected_loan else {
        gui.show_edit_dialog = false;
        return;
    };

    egui::Window::new("Edit Loan")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut gui.edit_name);

            ui.label("Original Amount");
            ui.text_edit_singleline(&mut gui.edit_amount);

            ui.label("Interest (%)");
            ui.text_edit_singleline(&mut gui.edit_interest);

            ui.label("Minimum Payment");
            ui.text_edit_singleline(&mut gui.edit_minimum);

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    gui.show_edit_dialog = false;
                }

                if ui.button("Save Changes").clicked() {
                    if let (Ok(amount), Ok(rate), Ok(minimum)) = (
                        gui.edit_amount.parse::<f64>(),
                        gui.edit_interest.parse::<f64>(),
                        gui.edit_minimum.parse::<f64>(),
                    ) {
                        if let Some(loan) = gui
                            .app
                            .portfolio_mut()
                            .loans_mut()
                            .iter_mut()
                            .find(|l| l.id() == id)
                        {
                            loan.set_name(gui.edit_name.clone());
                            loan.set_original_amount(amount);
                            loan.set_interest_rate(rate);
                            loan.set_minimum_payment(minimum);
                        }

                        gui.show_edit_dialog = false;
                    }
                }
            });
        });
}

pub fn save_dialog(ctx: &egui::Context, gui: &mut GuiApp) {
    if !gui.show_save_dialog {
        return;
    }

    egui::Window::new("Save Portfolio")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Filename");
            ui.text_edit_singleline(&mut gui.filename);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    gui.show_save_dialog = false;
                }

                if ui.button("Save").clicked() {
                    match json::save_portfolio(gui.app.portfolio(), &gui.filename) {
                        Ok(_) => gui.io_status = Some(format!("Saved to {}", gui.filename)),
                        Err(e) => gui.io_status = Some(format!("Failed to save: {}", e)),
                    }
                    gui.show_save_dialog = false;
                }
            });
        });
}

pub fn load_dialog(ctx: &egui::Context, gui: &mut GuiApp) {
    if !gui.show_load_dialog {
        return;
    }

    egui::Window::new("Load Portfolio")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Filename");
            ui.text_edit_singleline(&mut gui.filename);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    gui.show_load_dialog = false;
                }

                if ui.button("Load").clicked() {
                    match json::load_portfolio(&gui.filename) {
                        Ok(portfolio) => {
                            *gui.app.portfolio_mut() = portfolio;
                            gui.io_status = Some(format!("Loaded {}", gui.filename));
                            gui.invest_result = None; // Adjusted for unified feature
                            gui.selected_loan = None;
                        }
                        Err(e) => gui.io_status = Some(format!("Failed to load: {}", e)),
                    }
                    gui.show_load_dialog = false;
                }
            });
        });
}