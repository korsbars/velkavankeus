use crate::sim::strategy::PaymentStrategy;
use crate::sim::minimum::MinimumOnly;
use crate::sim::avalanche::Avalanche;
use crate::sim::snowball::Snowball;

pub fn strategies(extra: f64) -> Vec<Box<dyn PaymentStrategy>> {
    vec![
        Box::new(MinimumOnly),
        Box::new(Avalanche { extra_payment: extra }),
        Box::new(Snowball { extra_payment: extra }),
    ]
}

/// Strategies that make sense as the "pay extra toward debt" side of an
/// invest-vs-payoff comparison — everything except the no-extra baseline.
/// New strategies added to `strategies()` are picked up automatically.
pub fn accelerated_strategies(extra: f64) -> Vec<Box<dyn PaymentStrategy>> {
    strategies(extra)
        .into_iter()
        .filter(|s| s.name() != "Minimum Only")
        .collect()
}