pub use crate::alisases::*;
pub use crate::defaults::{DefaultBackProp, DefaultLazyTreePolicy, DefaultPlayout};
use crate::defaults::DefaultUctEvaluator;
pub use crate::ops::*;
pub use crate::traits::*;
pub use crate::tree_search::LazyMcts;

mod alisases;
mod defaults;
mod mcts_node;
mod ops;
mod traits;
mod tree_search;

/// This mcts uses UCT, naive simulation applying random moves until a final state, and scoring 1
/// if the player won.
pub type DefaultMcts<'a, State> = LazyMcts<'a,
    State,
    DefaultLazyTreePolicy<State, DefaultUctEvaluator, ()>,
    DefaultPlayout,
    DefaultBackProp,
    DefaultUctEvaluator,
    (),
>;
