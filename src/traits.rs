use std::fmt::Debug;
use std::ops::{Add, Div};

use ego_tree::{NodeId, Tree};
use num_traits::{ToPrimitive, Zero};

use crate::alisases::{LazyMctsNode, LazyMctsTree, Num};
use crate::mcts_node::MctsNode;

pub trait GameTrait: Clone {
    type Player: Debug + Clone;
    type Move: Debug + Clone;
    type Reward: Debug + Clone + Zero + ToPrimitive + Add + Div;

    fn legals_moves(&self) -> Vec<Self::Move>;

    fn player_turn(&self) -> Self::Player;

    fn hash(&self) -> u64;

    fn is_final(&self) -> bool;

    fn do_move(&mut self, m: &Self::Move);
}

pub trait Evaluator<A: Clone + Default> {
    type State: GameTrait;
    type LeafEval: Clone + Add + Div + Zero + ToPrimitive;
    type Args;

    fn eval_child(
        child: &LazyMctsNode<Self::State, A>,
        turn: &<Self::State as GameTrait>::Player,
        args: &Self::Args,
    ) -> Num;
    fn evaluate_leaf(
        child: &Self::State,
        turn: &<Self::State as GameTrait>::Player,
    ) -> Self::LeafEval;
}

pub trait Playout<State> {
    type Args;
    fn playout(state: State, args: Self::Args) -> State;
}

pub trait LazyTreePolicy<EV: Evaluator<A>, A: Clone + Default> {
    fn tree_policy(tree: &mut LazyMctsTree<EV::State, A>, root_state: EV::State) -> (NodeId, EV::State);
    fn update_state(
        root_state: EV::State,
        historic: &[<EV::State as GameTrait>::Move],
    ) -> EV::State;

    fn best_child(
        tree: &LazyMctsTree<EV::State, A>,
        turn: &<EV::State as GameTrait>::Player,
        parent_id: NodeId,
    ) -> NodeId;
}

pub trait BackPropPolicy<
    T: Clone,
    Move: Clone,
    R: Clone + Add + Div + Zero + ToPrimitive,
    A: Clone + Default,
    PlayoutResult,
>
{
    fn backprop(tree: &mut Tree<MctsNode<T, Move, R, A>>, leaf: NodeId, reward: PlayoutResult);
}
