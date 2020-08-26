use noisy_float::prelude::{n64, Float};

use crate::alisases::{Nat, Num};

/// Calculates the uct_value.
#[inline]
pub fn uct_value(parent_visits: Nat, sum_rewards: Num, node_visit: Nat, c: f64) -> Num {
    let exploitation_param = sum_rewards / node_visit as f64;
    let exploratation_param = (n64(parent_visits as f64).ln() / (node_visit as f64)).sqrt();
    exploitation_param + n64(c) * exploratation_param
}

#[test]
fn test_uct_value() {
    assert!((uct_value(500, Num::new(0.), 10, 2.0.sqrt()) - 1.339088).abs() < 0.00001)
}
