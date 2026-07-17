use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    id: u64,
    name: String,
    original_amount: f64,
    remaining_balance: f64,
    interest_rate: f64,
    minimum_payment: f64,
    months_elapsed: u64,
    total_interest_paid: f64,
}

#[derive(Debug,Clone,Copy)]
pub enum MonthlyAction {
    Minimum,
    MinimumPlus(f64),
    Payment(f64),
    Skip,
}

impl Loan {
    /// Creates a new Loan instance
    pub fn new(
        id: u64,
        name: String,
        amount: f64,
        interest_rate: f64,
        minimum_payment: f64,
    ) -> Self {
        Self {
            id,
            name,
            original_amount: amount,
            remaining_balance: amount,
            interest_rate,
            total_interest_paid: 0.0,
            minimum_payment,
            months_elapsed: 0,
        }
    }

    /// Advances the loan simulation by one month based on the action taken
    pub fn advance_month(&mut self, action: MonthlyAction) {
        let interest_this_month =
            self.remaining_balance * (self.interest_rate / 100.0) / 12.0;

        self.total_interest_paid += interest_this_month;
        self.remaining_balance += interest_this_month;

        let payment = match action {
            MonthlyAction::Minimum => self.minimum_payment,
            MonthlyAction::MinimumPlus(extra) => {
                self.minimum_payment + extra
            }
            MonthlyAction::Payment(amount) => amount,
            MonthlyAction::Skip => 0.0,
        };

        self.remaining_balance -= payment;
        if self.remaining_balance <= 0.0 {
            self.remaining_balance = 0.0;
        }
        self.months_elapsed += 1;
    }

    /// Checks if the loan is completely paid off
    pub fn is_paid_off(&self) -> bool {
        self.remaining_balance == 0.0
    }
}

/// Getters for private fields
impl Loan {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn original_amount(&self) -> f64 {
        self.original_amount
    }

    pub fn remaining_balance(&self) -> f64 {
        self.remaining_balance
    }

    pub fn interest_rate(&self) -> f64 {
        self.interest_rate
    }

    pub fn minimum_payment(&self) -> f64 {
        self.minimum_payment
    }

    pub fn total_interest_paid(&self) -> f64 {
        self.total_interest_paid
    }

    pub fn months_elapsed(&self) -> u64 {
        self.months_elapsed
    }
}


impl Loan {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_original_amount(&mut self, amount: f64) {
        self.original_amount = amount;
    }

    pub fn set_interest_rate(&mut self, rate: f64) {
        self.interest_rate = rate;
    }

    pub fn set_minimum_payment(&mut self, minimum: f64) {
        self.minimum_payment = minimum;
    }
}