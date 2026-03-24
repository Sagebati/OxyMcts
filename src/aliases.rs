use crate::mcts_node::MctsNode;
use crate::traits::GameTrait;
use bon::{bon, builder};
use ego_tree::Tree;
use noisy_float::prelude::N64;
use std::fs::Metadata;
use std::marker::PhantomData;
use std::ops::Deref;
use num_traits::Zero;

pub type Num = N64;
pub type Nat = u32;

pub type MctsTree<T, M, R, A> = Tree<MctsNode<T, M, R, A>>;
pub type LazyMctsTree<T, R, A> = Tree<LazyMctsNode<T, R, A>>;

type LazyMctsNodeAdapter<Game, Reward, Metadata> =
    MctsNode<Vec<<Game as GameTrait>::Move>, <Game as GameTrait>::Move, Reward, Metadata>;

pub struct LazyMctsNode<Game: GameTrait, Reward, Metadata> {
    node: LazyMctsNodeAdapter<Game, Reward, Metadata>,
    phantom_data: PhantomData<Game>,
}

#[bon]
impl<Game: GameTrait, Reward: Zero, Metadata: Default> LazyMctsNode<Game, Reward, Metadata> {
    #[builder]
    pub fn new(game: &Game, metadata: Metadata) -> Self {
        let moves = game.legals_moves();
        Self {
            node: MctsNode {
                sum_rewards: Reward::zero(),
                n_visits: 0,
                unvisited_moves: moves,
                hash: 0,
                state: vec![],
                additional_info: metadata,
            },
            phantom_data: Default::default(),
        }
    }
}

impl<Game: GameTrait, Reward, Metadata> Deref for LazyMctsNode<Game, Reward, Metadata> {
    type Target = LazyMctsNodeAdapter<Game, Reward, Metadata>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
