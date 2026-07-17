use crate::modeling::portfolio::Portfolio;
use crate::sim::strategy::PaymentStrategy;

pub struct StrategyRun {
    pub name: String,
    pub result: SimulationResult,
}

pub struct FinancialComparison {
    pub minimum: SimulationResult,
    pub payoff: SimulationResult,

    pub investment_series_minimum: Vec<(u64, f64)>,
    pub investment_series_payoff: Vec<(u64, f64)>,
    pub invested_if_minimum: f64,
    pub invested_if_payoff: f64,

    pub investment_series_minimum_no_reinvest: Vec<(u64, f64)>,
    pub investment_series_payoff_no_reinvest: Vec<(u64, f64)>,
    pub invested_if_minimum_no_reinvest: f64,
    pub invested_if_payoff_no_reinvest: f64,
}

pub struct Simulation {
    portfolio: Portfolio, 
    current_month: u64,
    name: String,
    strategy: Box<dyn PaymentStrategy>,    
}

impl Simulation {
    /// Creates a new simulation.
    pub fn new(
        portfolio: Portfolio,
        name: String,
        strategy: Box<dyn PaymentStrategy>,
    ) -> Self {
        Self {
            portfolio,
            current_month: 0, 
            name,
            strategy,
        }    
    }

    pub fn current_month(&self) -> u64 {
        self.current_month
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }

    pub fn advance_month(&mut self) {
        self.strategy.apply(&mut self.portfolio);
        self.current_month += 1;
    }

    /// Runs the simulation by advancing the month on every loan in the portfolio
    pub fn run(&mut self) -> SimulationResult {
        let mut history = Vec::new();

        history.push(Snapshot::new(
                0,
                self.portfolio.clone(),
        ));

        while !self.portfolio.all_paid_off() {
            self.advance_month();

            history.push(Snapshot::new(
                self.current_month,
                self.portfolio.clone(),
            ));
        }

        SimulationResult {
            months: self.current_month,
            total_interest_paid: self.portfolio.total_interest_paid(),
            total_paid: self.portfolio.total_original_amount()
                + self.portfolio.total_interest_paid(),
            history,
        }
    }
}

pub struct SimulationResult {
    months: u64,
    total_interest_paid: f64,
    total_paid: f64,
    history: Vec<Snapshot>,
}

impl SimulationResult {
    pub fn history(&self) -> &[Snapshot] {
        &self.history
    }

    pub fn months(&self) -> u64 {
        self.months
    }

    pub fn total_interest_paid(&self) -> f64 {
        self.total_interest_paid
    }

    pub fn total_paid(&self) -> f64 {
        self.total_paid
    }
}


pub enum SimulationMode {
    Summary,
    MonthByMonth,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    month: u64,
    portfolio: Portfolio,
}

impl Snapshot {
    pub fn new(month: u64, portfolio: Portfolio) -> Self {
        Self { month, portfolio }
    }

    pub fn month(&self) -> u64 {
        self.month
    }

    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }
}


