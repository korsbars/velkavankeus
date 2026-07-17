use velkavankaus_simulaattori::modeling::loan::{Loan, MonthlyAction};

/// Creates a loan for use in tests.
fn create_loan(
    id: u64,
    amount: f64,
    interest_rate: f64,
    minimum_payment: f64,
) -> Loan {
    Loan::new(
        id,
        format!("Loan {}", id),
        amount,
        interest_rate,
        minimum_payment,
    )
}

#[test]
fn new_loan_has_correct_initial_values() {
    let loan = create_loan(1, 1000.0, 12.0, 100.0);

    assert_eq!(loan.remaining_balance(), 1000.0);
    assert_eq!(loan.total_interest_paid(), 0.0);
    assert_eq!(loan.months_elapsed(), 0);
    assert!(!loan.is_paid_off());
}

#[test]
fn minimum_payment_reduces_balance() {
    let mut loan = create_loan(1, 1000.0, 0.0, 100.0);

    let before = loan.remaining_balance();

    loan.advance_month(MonthlyAction::Minimum);

    assert!(loan.remaining_balance() < before);
}

#[test]
fn skipping_payment_keeps_loan_unpaid() {
    let mut loan = create_loan(1, 1000.0, 12.0, 100.0);

    let before = loan.remaining_balance();

    loan.advance_month(MonthlyAction::Skip);

    assert!(loan.remaining_balance() > before);
    assert!(!loan.is_paid_off());
}

#[test]
fn overpayment_pays_off_loan() {
    let mut loan = create_loan(1, 100.0, 0.0, 50.0);

    loan.advance_month(MonthlyAction::Payment(500.0));

    assert!(loan.is_paid_off());
    assert_eq!(loan.remaining_balance(), 0.0);
}

#[test]
fn zero_interest_loan_takes_expected_months() {
    let mut loan = create_loan(1, 1000.0, 0.0, 100.0);

    while !loan.is_paid_off() {
        loan.advance_month(MonthlyAction::Minimum);
    }

    assert_eq!(loan.months_elapsed(), 10);
    assert_eq!(loan.total_interest_paid(), 0.0);
}