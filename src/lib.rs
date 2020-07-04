use crate::alisases::LazyMctsTree;
use crate::tree_search::LazyMcts;
use crate::defaults::{DefaultLazyTreePolicy, DefaultPlayout, DefaultBackProp};
use crate::traits::Evaluator;

mod mcts_node;
mod ops;
mod traits;
mod tree_search;
mod defaults;
mod alisases;

type DefaultMcts<E:Evaluator<()>> = LazyMcts<DefaultLazyTreePolicy<E, ()>,
    DefaultPlayout<E::State>, DefaultBackProp, E, ()>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
