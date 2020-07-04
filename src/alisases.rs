use crate::mcts_node::MctsNode;
use crate::traits::GameTrait;
use ego_tree::Tree;
use noisy_float::prelude::N64;

pub(crate) type Num = N64;
pub(crate) type Nat = u32;
pub(crate) type MctsTree<T, M, R, A> = Tree<MctsNode<T, M, R, A>>;

pub(crate) type LazyMctsNode<T, A> =
    MctsNode<Vec<<T as GameTrait>::Move>, <T as GameTrait>::Move, <T as GameTrait>::Reward, A>;
pub(crate) type LazyMctsTree<T, A> = Tree<LazyMctsNode<T, A>>;
