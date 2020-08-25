pub use crate::alisases::*;
use crate::defaults::DefaultUctEvaluator;
pub use crate::defaults::{DefaultBackProp, DefaultLazyTreePolicy, DefaultPlayout};
pub use crate::ops::*;
pub use crate::traits::*;
pub use crate::tree_search::LazyMcts;

mod alisases;
mod defaults;
mod mcts_node;
mod ops;
mod traits;
mod tree_search;

pub type DefaultMcts<State> = LazyMcts<
    State,
    DefaultLazyTreePolicy<State, DefaultUctEvaluator, ()>,
    DefaultPlayout,
    DefaultBackProp,
    DefaultUctEvaluator,
    (),
>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
