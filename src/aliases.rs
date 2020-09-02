use crate::mcts_node::MctsNode;
use crate::traits::GameTrait;
use ego_tree::Tree;
use noisy_float::prelude::N64;

pub type Num = N64;
pub type Nat = u32;

pub type MctsTree<T, M, R, A> = Tree<MctsNode<T, M, R, A>>;
pub type LazyMctsTree<T, R, A> = Tree<LazyMctsNode<T, R, A>>;

pub type LazyMctsNode<T, Reward, A> =
    MctsNode<Vec<<T as GameTrait>::Move>, <T as GameTrait>::Move, Reward, A>;

