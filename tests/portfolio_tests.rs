use velkavankaus_simulaattori::modeling::loan::Loan;
use velkavankaus_simulaattori::modeling::portfolio::Portfolio;

/// Creates a loan for use in tests.
fn create_loan(id: u64, amount: f64) -> Loan {
    Loan::new(
        id,
        format!("Loan {}", id),
        amount,
        5.0,
        200.0,
    )
}

#[test]
fn new_portfolio_is_empty() {
    let portfolio = Portfolio::new();

    assert!(portfolio.is_empty());
    assert_eq!(portfolio.loan_count(), 0);
}

#[test]
fn adding_one_loan_increases_count() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(create_loan(1, 15_000.0));

    assert_eq!(portfolio.loan_count(), 1);
    assert!(!portfolio.is_empty());
}

#[test]
fn adding_multiple_loans_increases_count() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(create_loan(1, 15_000.0));
    portfolio.add_loan(create_loan(2, 8_000.0));
    portfolio.add_loan(create_loan(3, 250_000.0));

    assert_eq!(portfolio.loan_count(), 3);
}

#[test]
fn removing_existing_loan_returns_true() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(create_loan(1, 15_000.0));

    let removed = portfolio.remove_loan(1);

    assert!(removed);
    assert_eq!(portfolio.loan_count(), 0);
    assert!(portfolio.is_empty());
}

#[test]
fn removing_nonexistent_loan_returns_false() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(create_loan(1, 15_000.0));

    let removed = portfolio.remove_loan(42);

    assert!(!removed);
    assert_eq!(portfolio.loan_count(), 1);
}

#[test]
fn removing_one_of_many_loans_keeps_the_others() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(create_loan(1, 15_000.0));
    portfolio.add_loan(create_loan(2, 8_000.0));
    portfolio.add_loan(create_loan(3, 250_000.0));

    assert!(portfolio.remove_loan(2));

    assert_eq!(portfolio.loan_count(), 2);
}

#[test]
fn removing_same_loan_twice_only_succeeds_once() {
    let mut portfolio = Portfolio::new();

    portfolio.add_loan(create_loan(1, 15_000.0));

    assert!(portfolio.remove_loan(1));
    assert!(!portfolio.remove_loan(1));
}

#[test]
fn empty_portfolio_reports_zero_loans() {
    let portfolio = Portfolio::new();

    assert_eq!(portfolio.loan_count(), 0);
    assert!(portfolio.is_empty());
}