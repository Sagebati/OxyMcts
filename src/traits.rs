use std::fmt::Debug;
use std::ops::{Add, Div};

use ego_tree::{NodeId, Tree};
use num_traits::{ToPrimitive, Zero};

use crate::alisases::{LazyMctsNode, LazyMctsTree, Num};
use crate::mcts_node::MctsNode;

pub trait GameTrait: Clone {
    type Player: Debug + Clone + Eq;
    type Move: Debug + Clone;

    fn legals_moves(&self) -> Vec<Self::Move>;

    fn player_turn(&self) -> Self::Player;

    fn hash(&self) -> u64;

    fn is_final(&self) -> bool;

    fn do_move(&mut self, m: &Self::Move);

    fn get_winner(&self) -> Self::Player;
}

pub trait EvaluatorBis<State: GameTrait, AdditionalInfo: Clone + Default> {
    type Args;
    type Reward: Clone + Add + Div + Zero + ToPrimitive;

    fn eval_child(
        child: &LazyMctsNode<State, Self::Reward, AdditionalInfo>,
        turn: &State::Player,
        args: &Self::Args,
    ) -> Num;
    fn evaluate_leaf(child: State, turn: &State::Player) -> Self::Reward;
}

pub trait Playout<State> {
    type Args;
    fn playout(state: State, args: Self::Args) -> State;
}

pub trait LazyTreePolicy<State: GameTrait, EV: EvaluatorBis<State, A>, A: Clone + Default> {
    fn tree_policy(
        tree: &mut LazyMctsTree<State, EV::Reward, A>,
        root_state: State,
    ) -> (NodeId, State);

    fn update_state(root_state: State, historic: &[State::Move]) -> State;

    fn best_child(
        tree: &LazyMctsTree<State, EV::Reward, A>,
        turn: &State::Player,
        parent_id: NodeId,
    ) -> NodeId;
}

pub trait BackPropPolicy<
    State: Clone,
    Move: Clone,
    Reward: Clone + Add + Div + Zero + ToPrimitive,
    AdditionalInfo: Clone + Default,
    PlayoutResult,
>
{
    fn backprop(
        tree: &mut Tree<MctsNode<State, Move, Reward, AdditionalInfo>>,
        leaf: NodeId,
        reward: PlayoutResult,
    );
}
