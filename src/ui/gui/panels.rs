use eframe::egui;
use crate::ui::gui::window::GuiApp;
use crate::ui::gui::dialogs;
use crate::sim::{invest, registry};

// Helper function to format numbers precisely as: 271 202,23€
fn format_euro(val: f64) -> String {
    let formatted = format!("{:.2}", val);
    let parts: Vec<&str> = formatted.split('.').collect();
    let int_part = parts[0];
    let frac_part = parts[1];
    
    let is_neg = int_part.starts_with('-');
    let digits = if is_neg { &int_part[1..] } else { int_part };
    
    let mut result = String::new();
    for (i, c) in digits.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            result.push(' '); // Space as thousands separator
        }
        result.push(c);
    }
    
    let reversed_int: String = result.chars().rev().collect();
    let sign = if is_neg { "-" } else { "" };
    
    // Comma as decimal separator, and € symbol tightly glued to the end
    format!("{}{},{}€", sign, reversed_int, frac_part)
}

pub fn top_toolbar(ui: &mut egui::Ui, gui: &mut GuiApp) {
    ui.horizontal(|ui| {
        if ui.button("➕ Add Loan").clicked() {
            gui.show_add_dialog = true;
        }

        if ui.button("💾 Save").clicked() {
            if let Some(path_buf) = rfd::FileDialog::new()
                .add_filter("JSON Portfolio", &["json"])
                .set_title("Save Loan Portfolio")
                .save_file() 
            {
                let path_str = path_buf.to_string_lossy();
                // Call your exact save_portfolio function here
                match crate::storage::json::save_portfolio(gui.app.portfolio(), &path_str) {
                    Ok(_) => gui.io_status = Some("Portfolio saved successfully!".to_string()),
                    Err(e) => gui.io_status = Some(format!("Save failed: {}", e)),
                }
            }
        }

        if ui.button("📂 Load").clicked() {
            if let Some(path_buf) = rfd::FileDialog::new()
                .add_filter("JSON Portfolio", &["json"])
                .set_title("Open Loan Portfolio")
                .pick_file() 
            {
                let path_str = path_buf.to_string_lossy();
                // Call your exact load_portfolio function here
                match crate::storage::json::load_portfolio(&path_str) {
                    Ok(loaded_portfolio) => {
                        *gui.app.portfolio_mut() = loaded_portfolio;
                        gui.selected_loan = None; // Reset selection safety
                        gui.io_status = Some("Portfolio loaded successfully!".to_string());
                    }
                    Err(e) => gui.io_status = Some(format!("Load failed: {}", e)),
                }
            }
        }

        if let Some(status) = &gui.io_status {
            ui.separator();
            ui.label(status);
        }
    });
}

pub fn portfolio_panel(ui: &mut egui::Ui, gui: &mut GuiApp) {
    ui.heading("Portfolio Grid");
    ui.separator();

    if gui.app.portfolio().is_empty() {
        ui.label("No active loans in the portfolio.");
        return;
    }

    egui::Grid::new("portfolio_loans_grid")
        .num_columns(5)
        .spacing([15.0, 8.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Loan Name").strong());
            ui.label(egui::RichText::new("Balance").strong());
            ui.label(egui::RichText::new("Interest").strong());
            ui.label(egui::RichText::new("Minimum").strong());
            ui.label(egui::RichText::new("Actions").strong());
            ui.end_row();

            let loans_to_render: Vec<_> = gui.app.portfolio().loans().iter().cloned().collect();

            for loan in loans_to_render {
                let is_selected = gui.selected_loan == Some(loan.id());
                
                if ui.selectable_label(is_selected, loan.name()).clicked() {
                    gui.selected_loan = Some(loan.id());
                }

                ui.label(format_euro(loan.remaining_balance()));
                ui.label(format!("{:.2}%", loan.interest_rate()));
                ui.label(format_euro(loan.minimum_payment()));
                
                ui.horizontal(|ui| {
                    if ui.button("✏").on_hover_text("Edit Loan").clicked() {
                        gui.selected_loan = Some(loan.id());
                        dialogs::open_edit_dialog(gui);
                    }
                    if ui.button("🗑").on_hover_text("Delete Loan").clicked() {
                        gui.app.portfolio_mut().remove_loan(loan.id());
                        if gui.selected_loan == Some(loan.id()) {
                            gui.selected_loan = None;
                        }
                    }
                });
                ui.end_row();
            }
        });
}

pub fn details_panel(ui: &mut egui::Ui, gui: &mut GuiApp) {
    ui.heading("Portfolio Overview");
    ui.separator();

    let portfolio = gui.app.portfolio();

    ui.label(format!("Total Active Loans: {}", portfolio.loan_count()));
    ui.label(format!("Aggregated Liability: {}", format_euro(portfolio.total_balance())));
    ui.label(format!("Cumulative Interest Paid: {}", format_euro(portfolio.total_interest_paid())));
}

pub fn invest_panel(ui: &mut egui::Ui, gui: &mut GuiApp) {
    ui.heading("Invest vs. Pay Off Faster");
    ui.separator();

    ui.label(
        "Compare paying structural minimums and routing the difference to investments, \
         versus aggressively accelerating your debt payoff schedule and deploying freed resources later.",
    );

    ui.horizontal(|ui| {
        ui.label("Extra payment/month:");
        ui.text_edit_singleline(&mut gui.invest_extra);
        ui.label("€");
    });

    ui.horizontal(|ui| {
        ui.label("Assumed annual return (%):");
        ui.text_edit_singleline(&mut gui.invest_return_pct);
    });

    let strategies = registry::accelerated_strategies(0.0);
    let names: Vec<&str> = strategies.iter().map(|s| s.name()).collect();

    if !names.is_empty() {
        let index = gui.invest_strategy_index.min(names.len() - 1);

        ui.horizontal(|ui| {
            ui.label("Payoff strategy:");
            egui::ComboBox::from_id_salt("invest_strategy")
                .selected_text(names[index])
                .show_ui(ui, |ui| {
                    for (i, name) in names.iter().enumerate() {
                        // FIX: Track by strategy array index literal key 'i', not layout slice label payload
                        ui.selectable_value(&mut gui.invest_strategy_index, i, *name);
                    }
                });
        });
    }

    let can_run = !gui.app.portfolio().is_empty() && !names.is_empty();

    if can_run {
        let extra = gui.invest_extra.parse::<f64>().unwrap_or(0.0);
        let annual_return = gui.invest_return_pct.parse::<f64>().unwrap_or(0.0);
        let index = gui.invest_strategy_index.min(names.len() - 1);
        let strategy = registry::accelerated_strategies(extra).remove(index);

        gui.invest_result = Some(invest::compare(
            gui.app.portfolio(),
            strategy,
            extra,
            annual_return,
            gui.invest_only_extra,
        ));
    } else {
        gui.invest_result = None;
    }

    ui.separator();

    if let Some(r) = &gui.invest_result {
        let strategy_name = if !names.is_empty() {
            names[gui.invest_strategy_index.min(names.len() - 1)]
        } else {
            "Payoff strategy"
        };

        let min_interest = r.minimum.total_interest_paid();
        let payoff_interest = r.payoff.total_interest_paid();
        let interest_saved = min_interest - payoff_interest;

        ui.label(format!(
            "Minimum only baseline duration: {} months (Total Interest: {})",
            r.minimum.months(), format_euro(min_interest)
        ));
        ui.label(format!(
            "{} accelerated duration: {} months (Total Interest: {})",
            strategy_name, r.payoff.months(), format_euro(payoff_interest)
        ));
        ui.label(format!("Interest saved across timeline by paying extra: {}", format_euro(interest_saved)));

        ui.separator();
        ui.heading("Investment Strategy Matrix");
        ui.add_space(4.0);

        egui::Grid::new("calculation_matrix_grid")
            .num_columns(3)
            .spacing([25.0, 15.0])
            .striped(true)
            .show(ui, |ui| {
                // Header row - Shortened to prevent column stretching alignment bugs
                ui.label("");
                ui.label(egui::RichText::new("Reinvest Freed Cash").strong());
                ui.label(egui::RichText::new("Flat Extra Only").strong());
                ui.end_row();

                // Row 1: Gross Investment Wealth
                ui.label(egui::RichText::new("Gross Investment Balance").strong());
                ui.label(format_euro(r.invested_if_minimum));
                ui.label(format_euro(r.invested_if_minimum_no_reinvest));
                ui.end_row();

                // Row 2: Total Interest Lost
                ui.label(egui::RichText::new("Minus Total Interest Paid").strong());
                ui.label(format_euro(min_interest));
                ui.label(format_euro(min_interest));
                ui.end_row();

                ui.label(egui::RichText::new("Net Position (Minimums)").small());
                ui.label(format_euro(r.invested_if_minimum - min_interest));
                ui.label(format_euro(r.invested_if_minimum_no_reinvest - min_interest));
                ui.end_row();

                // Separator Row inside Grid
                ui.label("---------------------------");
                ui.label("---------------------------");
                ui.label("---------------------------");
                ui.end_row();

                // Row 3: Accelerated Gross Wealth
                ui.label(egui::RichText::new(format!("Gross Balance ({} Strategy)", strategy_name)).strong());
                ui.label(format_euro(r.invested_if_payoff));
                ui.label(format_euro(r.invested_if_payoff_no_reinvest));
                ui.end_row();

                // Row 4: Accelerated Interest Lost
                ui.label(egui::RichText::new("Minus Total Interest Paid").strong());
                ui.label(format_euro(payoff_interest));
                ui.label(format_euro(payoff_interest));
                ui.end_row();

                // Row 5: Accelerated Net Position
                ui.label(egui::RichText::new("Net Position (Accelerated)").small());
                ui.label(format_euro(r.invested_if_payoff - payoff_interest));
                ui.label(format_euro(r.invested_if_payoff_no_reinvest - payoff_interest));
                ui.end_row();
            });
    } else {
        ui.label("Add a loan to see comparison results automatically.");
    }
}

pub fn charts_panel(ui: &mut egui::Ui, gui: &mut GuiApp) {
    ui.heading("Charts");
    ui.separator();

    if let Some(invest_result) = &gui.invest_result {
        ui.horizontal(|ui| {
            ui.label("Filter Graph Series:");
            ui.checkbox(&mut gui.show_debt_charts, "📉 Debt Overviews");
            ui.checkbox(&mut gui.show_reinvest_charts, "🔄 Reinvestment Strategies");
            ui.checkbox(&mut gui.show_flat_charts, "Flat Extra Only Strategies");
        });
        ui.add_space(8.0);

        crate::ui::gui::charts::invest_chart(ui, invest_result, gui.show_debt_charts, gui.show_reinvest_charts, gui.show_flat_charts);
    } else {
        ui.label("Add a loan to see the compound interest growth graphs automatically.");
    }
}