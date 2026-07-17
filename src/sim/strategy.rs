use crate::modeling::portfolio::Portfolio;

pub trait PaymentStrategy {
    fn apply(&self, portfolio: &mut Portfolio);

    fn name(&self) -> &'static str;
}
