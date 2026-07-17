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
