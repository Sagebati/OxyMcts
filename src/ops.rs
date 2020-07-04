use noisy_float::prelude::{Float, n64};

use crate::alisases::{Nat, Num};

/// Calculates the uct_value.
#[inline]
pub fn uct_value(parent_visits: Nat, sum_rewards: Num, node_visit: Nat) -> Num {
    let exploration_param: Num = n64(2.0).sqrt();
    let mean = sum_rewards / node_visit as f64;
    let exploitation_param = (n64(parent_visits as f64).ln() / (node_visit as f64)).sqrt();
    mean + exploration_param * exploitation_param
}

#[test]
fn test_uct_value() {
    assert!((uct_value(500, Num::new(0.), 10) - 1.339088).abs() < 0.00001)
}
