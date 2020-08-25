use std::fmt::Debug;
use std::ops::{Add, Div};

use ego_tree::{NodeId, Tree};
use num_traits::{ToPrimitive, Zero};

use crate::alisases::{LazyMctsNode, LazyMctsTree, Num};
use crate::mcts_node::MctsNode;

pub trait GameTrait: Clone {
    type Player: Debug + Clone + Eq;
    type Move: Debug + Clone;

    /// Returns a list of legal_move, for the actual player.
    fn legals_moves(&self) -> Vec<Self::Move>;

    /// Return the player actually playing.
    fn player_turn(&self) -> Self::Player;

    /// Return if any the hash of the game state.
    fn hash(&self) -> u64;

    /// If the game is finished this return true.
    fn is_final(&self) -> bool;

    /// Play the move and alter the state.
    fn do_move(&mut self, m: &Self::Move);

    /// If the game is finished this function returns the winner of the game.
    fn get_winner(&self) -> Self::Player;
}

pub trait Evaluator<State: GameTrait, AdditionalInfo: Clone + Default> {
    type Args;
    type Reward: Clone + Add + Div + Zero + ToPrimitive;

    /// Evaluates each node of the monte carlo tree search.
    /// for ex: using UCT
    fn eval_child(
        child: &LazyMctsNode<State, Self::Reward, AdditionalInfo>,
        turn: &State::Player,
        args: &Self::Args,
    ) -> Num;

    /// Evaluates the a final state, when a simulation is over when call this function to know
    /// the reward.
    fn evaluate_leaf(child: State, turn: &State::Player) -> Self::Reward;
}

pub trait Playout<State> {
    type Args;
    /// Plays the state to have a final state.
    fn playout(state: State, args: Self::Args) -> State;
}

pub trait LazyTreePolicy<State: GameTrait, EV: Evaluator<State, A>, A: Clone + Default> {
    /// Choose the best node, for example we apply the UCT to choose the best node then we expand
    /// it and we return the expansion.
    fn tree_policy(
        tree: &mut LazyMctsTree<State, EV::Reward, A>,
        root_state: State,
    ) -> (NodeId, State);

    /// This method is only needed because we don't store the state in each node so we need, to
    /// update the state with the stored historic in each node before simulating or expanding it.
    fn update_state(mut root_state: State, historic: &[State::Move]) -> State {
        for x in historic {
            root_state.do_move(x)
        }
        return root_state
    }

    /// This method use the Evaluator to get best child using evaluate_child.
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
