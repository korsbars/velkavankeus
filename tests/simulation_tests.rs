use velkavankaus_simulaattori::modeling::loan::{Loan, MonthlyAction};
use velkavankaus_simulaattori::modeling::portfolio::Portfolio;
use velkavankaus_simulaattori::sim::simulation::Simulation;
use velkavankaus_simulaattori::sim::strategy::PaymentStrategy;

/// A simple strategy that always makes the minimum payment.
struct MinimumOnly;

impl PaymentStrategy for MinimumOnly {
    fn apply(&self, portfolio: &mut Portfolio) {
        for loan in portfolio.loans_mut() {
            if !loan.is_paid_off() {
                loan.advance_month(MonthlyAction::Minimum);
            }
        }
    }
}

#[test]
fn simulation_can_be_created() {
    let portfolio = Portfolio::new();

    let _simulation = Simulation::new(
        portfolio,
        "Test".to_string(),
        Box::new(MinimumOnly),
    );
}

#[test]
fn simulation_runs_zero_interest_loan() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(Loan::new(
        1,
        "Test Loan".to_string(),
        1000.0,
        0.0,
        100.0,
    ));

    let mut simulation = Simulation::new(
        portfolio,
        "Zero Interest".to_string(),
        Box::new(MinimumOnly),
    );

    let result = simulation.run();

    assert_eq!(result.months(), 10);
    assert_eq!(result.total_interest_paid(), 0.0);
    assert_eq!(result.total_paid(), 1000.0);
}

#[test]
fn simulation_runs_interest_bearing_loan() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(Loan::new(
        1,
        "Interest Loan".to_string(),
        1000.0,
        12.0,
        500.0,
    ));

    let mut simulation = Simulation::new(
        portfolio,
        "Interest".to_string(),
        Box::new(MinimumOnly),
    );

    let result = simulation.run();

    assert!(result.months() > 0);
    assert!(result.total_interest_paid() > 0.0);
    assert!(result.total_paid() > 1000.0);
}

#[test]
fn empty_portfolio_finishes_immediately() {
    let portfolio = Portfolio::new();

    let mut simulation = Simulation::new(
        portfolio,
        "Empty".to_string(),
        Box::new(MinimumOnly),
    );

    let result = simulation.run();

    assert_eq!(result.months(), 0);
    assert_eq!(result.total_interest_paid(), 0.0);
    assert_eq!(result.total_paid(), 0.0);
}