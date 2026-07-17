use crate::modeling::loan::MonthlyAction;
use crate::modeling::portfolio::Portfolio;
use crate::sim::strategy::PaymentStrategy;

pub struct Avalanche {
    pub extra_payment: f64,
}

impl PaymentStrategy for Avalanche {
    fn apply(&self, portfolio: &mut Portfolio) {
        let target = portfolio.highest_interest_index();

        for (i, loan) in portfolio.loans_mut().iter_mut().enumerate() {
            if loan.is_paid_off() {
                continue;
            }

            if Some(i) == target {
                loan.advance_month(
                    MonthlyAction::MinimumPlus(self.extra_payment),
                );
            } else {
                loan.advance_month(MonthlyAction::Minimum);
            }
        }
    }

    fn name(&self) -> &'static str {
    "Avalanche"
    }
}
