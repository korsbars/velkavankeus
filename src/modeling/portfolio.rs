use crate::modeling::loan::Loan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    loans: Vec<Loan>,
}

impl Portfolio {
    /// Creates a new, empty Portfolio.
    pub fn new() -> Self {
        Self {
            loans: Vec::new(),
        }
    }

    /// Returns access to all loans.
    pub fn loans(&self) -> &[Loan] {
        &self.loans
    }

    /// Returns mutable access to all loans.
    pub fn loans_mut(&mut self) -> &mut [Loan] {
        &mut self.loans
    }

    /// Adds a loan to the portfolio.
    pub fn add_loan(&mut self, loan: Loan) {
        self.loans.push(loan);
    }

    /// Removes a loan from the portfolio.
    pub fn remove_loan(&mut self, id: u64) -> bool {
        let initial_length = self.loans.len();
        self.loans.retain(|loan| loan.id() != id);
        self.loans.len() < initial_length
    }

    /// Returns the total number of loans in the portfolio.
    pub fn loan_count(&self) -> usize {
        self.loans.len()
    }

    /// Returns true if the portfolio contains no loans.
    pub fn is_empty(&self) -> bool {
        self.loans.is_empty()
    }

    /// Returns the total remaining balance of every loan. 
    pub fn total_balance(&self) -> f64 {
        self.loans.iter().map(|loan| loan.remaining_balance()).sum()
    }

    /// Returns true if every loan in the portfolio has been paid off.
    pub fn all_paid_off(&self) -> bool {
        self.loans.iter().all(|loan| loan.is_paid_off())
    }

    /// Returns the total original principal of all loans.
    pub fn total_original_amount(&self) -> f64 {
        self.loans
            .iter()
            .map(|loan| loan.original_amount())
            .sum()
    }

    /// Returns the total interest paid across all loans.
    pub fn total_interest_paid(&self) -> f64 {
        self.loans
            .iter()
            .map(|loan| loan.total_interest_paid())
            .sum()
    }

    pub fn highest_interest_index(&self) -> Option<usize> {
    self.loans
        .iter()
        .enumerate()
        .filter(|(_, loan)| !loan.is_paid_off())
        .max_by(|(_, a), (_, b)| {
            a.interest_rate()
                .partial_cmp(&b.interest_rate())
                .unwrap()
        })
        .map(|(i, _)| i)
    }

    pub fn smallest_balance_index(&self) -> Option<usize> {
        self.loans
            .iter()
            .enumerate()
            .filter(|(_, loan)| !loan.is_paid_off())
            .min_by(|(_, a), (_, b)| {
                a.remaining_balance()
                    .partial_cmp(&b.remaining_balance())
                    .unwrap()
                })
            .map(|(i, _)| i)
    }
}
