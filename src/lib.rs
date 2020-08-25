pub use crate::alisases::*;
pub use crate::tree_search::{LazyMcts};
pub use crate::defaults::{DefaultLazyTreePolicy, DefaultPlayout, DefaultBackProp};
pub use crate::traits::*;
pub use crate::ops::*;

mod mcts_node;
mod ops;
mod traits;
mod tree_search;
mod defaults;
mod alisases;

pub type DefaultMcts<E: Evaluator<()>> = LazyMcts<DefaultLazyTreePolicy<E, ()>,
    DefaultPlayout, DefaultBackProp, E, ()>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
