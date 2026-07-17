use egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::sim::simulation::FinancialComparison; 

pub fn invest_chart(
    ui: &mut Ui, 
    result: &FinancialComparison,
    show_debt: bool,
    show_reinvest: bool,
    show_flat: bool,
) {
    Plot::new("invest_vs_payoff")
        .height(450.0)
        .legend(Default::default())
        .show(ui, |plot_ui| {
            // 1. DEBT OVERVIEWS
            if show_debt {
                let debt_minimum_points: PlotPoints = result
                    .minimum
                    .history()
                    .iter()
                    .map(|snap| [snap.month() as f64, snap.portfolio().total_balance()])
                    .collect();

                let debt_payoff_points: PlotPoints = result
                    .payoff
                    .history()
                    .iter()
                    .map(|snap| [snap.month() as f64, snap.portfolio().total_balance()])
                    .collect();

                plot_ui.line(Line::new("Debt Balance (Minimums Only)", debt_minimum_points).width(2.0_f32));
                plot_ui.line(Line::new("Debt Balance (Accelerated Payoff)", debt_payoff_points).width(2.0_f32));
            }

            // 2. REINVESTMENT STRATEGIES
            if show_reinvest {
                let invest_min_points: PlotPoints = result
                    .investment_series_minimum
                    .iter()
                    .map(|(m, v)| [*m as f64, *v])
                    .collect();

                let invest_payoff_points: PlotPoints = result
                    .investment_series_payoff
                    .iter()
                    .map(|(m, v)| [*m as f64, *v])
                    .collect();

                plot_ui.line(Line::new("Investments (Minimums + Reinvest)", invest_min_points).width(2.0_f32));
                plot_ui.line(Line::new("Investments (Payoff + Reinvest)", invest_payoff_points).width(2.0_f32));
            }

            // 3. FLAT EXTRA ONLY STRATEGIES
            if show_flat {
                let invest_min_no_reinvest_points: PlotPoints = result
                    .investment_series_minimum_no_reinvest
                    .iter()
                    .map(|(m, v)| [*m as f64, *v])
                    .collect();

                let invest_payoff_no_reinvest_points: PlotPoints = result
                    .investment_series_payoff_no_reinvest
                    .iter()
                    .map(|(m, v)| [*m as f64, *v])
                    .collect();

                plot_ui.line(Line::new("Investments (Minimums + Flat Extra Only)", invest_min_no_reinvest_points).width(1.5_f32));
                plot_ui.line(Line::new("Investments (Payoff + Flat Extra Only)", invest_payoff_no_reinvest_points).width(1.5_f32));
            }
        });
}