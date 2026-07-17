use crate::modeling::loan::MonthlyAction;
use crate::modeling::portfolio::Portfolio;
use crate::sim::strategy::PaymentStrategy;

pub struct MinimumOnly;

impl PaymentStrategy for MinimumOnly {
    fn apply(&self, portfolio: &mut Portfolio) {
        for loan in portfolio.loans_mut() {
            if !loan.is_paid_off() {
                loan.advance_month(MonthlyAction::Minimum);
            }
        }
    }
    fn name(&self) ->&'static str {
        "Minimum Only"
    }
}
