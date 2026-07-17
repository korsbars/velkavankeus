use crate::modeling::portfolio::Portfolio;
use crate::sim::strategy::PaymentStrategy;
use crate::sim::minimum::MinimumOnly;
use crate::sim::simulation::{Simulation, SimulationResult, FinancialComparison};

// Bringing your structure into the engine scope
#[derive(Clone)]
pub struct InvestmentAccount {
    balance: f64,
    annual_return: f64,
}

impl InvestmentAccount {
    pub fn new(rate: f64) -> Self {
        Self {
            balance: 0.0,
            annual_return: rate,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn advance_month(&mut self) {
        let monthly = self.annual_return / 12.0 / 100.0;
        self.balance *= 1.0 + monthly;
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

pub fn compare(
    portfolio: &Portfolio,
    strategy: Box<dyn PaymentStrategy>,
    extra_payment: f64,
    annual_return_pct: f64,
    _only_invest_extra: bool,
) -> FinancialComparison {
    let strategy_name = strategy.name().to_string();

    let mut min_sim = Simulation::new(
        portfolio.clone(),
        MinimumOnly.name().to_string(),
        Box::new(MinimumOnly),
    );
    let minimum_result = min_sim.run();

    let mut payoff_sim = Simulation::new(
        portfolio.clone(),
        strategy_name.clone(),
        strategy,
    );
    let payoff_result = payoff_sim.run();

    let horizon = minimum_result.months() as usize;

    let budget: f64 = portfolio
        .loans()
        .iter()
        .map(|l| l.minimum_payment())
        .sum::<f64>()
        + extra_payment;

    // 1. Standard scenarios (With Reinvestment)
    let investment_series_minimum =
        simulate_investment_series(&minimum_result, budget, extra_payment, horizon, annual_return_pct, false, true);
    let investment_series_payoff =
        simulate_investment_series(&payoff_result, budget, extra_payment, horizon, annual_return_pct, false, false);

    // 2. Strict scenarios (Without Reinvestment)
    let investment_series_minimum_no_reinvest =
        simulate_investment_series(&minimum_result, budget, extra_payment, horizon, annual_return_pct, true, true);
    let investment_series_payoff_no_reinvest =
        simulate_investment_series(&payoff_result, budget, extra_payment, horizon, annual_return_pct, true, false);

    let invested_if_minimum = investment_series_minimum.last().map(|(_, v)| *v).unwrap_or(0.0);
    let invested_if_payoff = investment_series_payoff.last().map(|(_, v)| *v).unwrap_or(0.0);
    let invested_if_minimum_no_reinvest = investment_series_minimum_no_reinvest.last().map(|(_, v)| *v).unwrap_or(0.0);
    let invested_if_payoff_no_reinvest = investment_series_payoff_no_reinvest.last().map(|(_, v)| *v).unwrap_or(0.0);

    FinancialComparison {
        minimum: minimum_result,
        payoff: payoff_result,
        investment_series_minimum,
        investment_series_payoff,
        invested_if_minimum,
        invested_if_payoff,
        investment_series_minimum_no_reinvest,
        investment_series_payoff_no_reinvest,
        invested_if_minimum_no_reinvest,
        invested_if_payoff_no_reinvest,
    }
}

fn simulate_investment_series(
    sim_result: &SimulationResult,
    total_budget: f64,
    extra_payment: f64,
    horizon_months: usize,
    annual_return_pct: f64,
    no_reinvest_freed_up: bool,
    is_minimum_strategy: bool,
) -> Vec<(u64, f64)> {
    let mut series = Vec::new();
    let mut account = InvestmentAccount::new(annual_return_pct);

    for month in 0..=horizon_months {
        if month > 0 {
            account.advance_month();
        }

        let is_debt_active = month < sim_result.history().len() && sim_result.history()[month].portfolio().total_balance() > 0.0;

        let monthly_contribution = if no_reinvest_freed_up {
            if is_minimum_strategy {
                // Minimums Only (Flat Extra Only): You ONLY invest the raw extra_payment amount
                if month == 0 { 0.0 } else { extra_payment }
            } else {
                // Accelerated Payoff (Flat Extra Only): 0.0 while debt is active, then flat extra after debt is completely dead
                if is_debt_active || month == 0 { 0.0 } else { extra_payment }
            }
        } else {
            // Dynamic Snowball Selection Loop
            let active_debt_minimums = if month < sim_result.history().len() {
                sim_result.history()[month]
                    .portfolio()
                    .loans()
                    .iter()
                    .filter(|l| l.remaining_balance() > 0.01) // Correctly ignore paid-off loans
                    .map(|l| l.minimum_payment())
                    .sum::<f64>()
            } else {
                0.0
            };

            let paid_to_loans = if is_debt_active {
                if is_minimum_strategy {
                    active_debt_minimums
                } else {
                    active_debt_minimums + extra_payment
                }
            } else {
                0.0
            };

            let left_over = total_budget - paid_to_loans;
            if left_over > 0.0 { left_over } else { 0.0 }
        };

        if month > 0 {
            account.deposit(monthly_contribution);
        }
        series.push((month as u64, account.balance()));
    }

    series
}