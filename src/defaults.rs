use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div};

use ego_tree::{NodeId, NodeMut, Tree};
use noisy_float::prelude::n64;
use num_traits::{ToPrimitive, Zero};
use rand::{Rng, thread_rng};
use rand::prelude::IteratorRandom;

use crate::{Evaluator, Num, uct_value};
use crate::alisases::{LazyMctsNode, LazyMctsTree};
use crate::mcts_node::MctsNode;
use crate::traits::{BackPropPolicy, GameTrait, LazyTreePolicy, Playout};

/// A default backprop policy it will take the reward of the simulation and backkpropagate the
/// result  to the branch nodes.
pub struct DefaultBackProp {}

impl<
    T: Clone,
    Move: Clone,
    R: Add + AddAssign + Div + Clone + Zero + ToPrimitive,
    A: Clone + Default,
> BackPropPolicy<T, Move, R, A> for DefaultBackProp
{
    fn backprop(tree: &mut Tree<MctsNode<T, Move, R, A>>, leaf: NodeId, reward: R) {
        let root_id = tree.root().id();
        let mut current_node_id = leaf;
        // Update the branch
        while current_node_id != root_id {
            let mut node_to_update = tree.get_mut(current_node_id).unwrap();
            node_to_update.value().n_visits += 1;
            node_to_update.value().sum_rewards =
                node_to_update.value().sum_rewards.clone() + reward.clone();
            current_node_id = node_to_update.parent().unwrap().id();
        }
        // Update root
        let mut node_to_update = tree.get_mut(current_node_id).unwrap();
        node_to_update.value().n_visits += 1;
        node_to_update.value().sum_rewards += reward;
    }
}

/// Simulating taking random moves a applying until the end.
pub struct DefaultPlayout {}

impl<T: GameTrait> Playout<T> for DefaultPlayout {
    type Args = ();

    fn playout(mut state: T, _args: ()) -> T {
        while !state.is_final() {
            let m = state
                .legals_moves()
                .into_iter()
                .choose(&mut thread_rng())
                .unwrap();
            state.do_move(&m);
        }
        state
    }
}

/// Explores at least once each child node, before going deeper.
pub struct DefaultLazyTreePolicy<State: GameTrait, EV: Evaluator<State, A>, A: Clone + Default> {
    phantom_state: PhantomData<State>,
    phantom_a: PhantomData<A>,
    phantom_ev: PhantomData<EV>,
}

impl<State: GameTrait, EV: Evaluator<State, A, Args=u32>, A: Clone + Default>
DefaultLazyTreePolicy<State, EV, A>
{
    pub fn select(
        mut tree: &mut LazyMctsTree<State, EV::Reward, A>,
        turn: &State::Player,
    ) -> NodeId {
        let mut current_node_id = tree.root().id();
        while tree.get(current_node_id).unwrap().has_children() {
            if tree.get(current_node_id).unwrap().value().can_add_child() {
                return current_node_id;
            } else {
                current_node_id = Self::best_child(&mut tree, turn, current_node_id);
            }
        }
        current_node_id
    }

    pub fn expand(
        mut node_to_expand: NodeMut<LazyMctsNode<State, EV::Reward, A>>,
        root_state: State,
    ) -> (NodeId, State) {
        let mut new_state = Self::update_state(root_state, &node_to_expand.value().state);
        if !node_to_expand.value().can_add_child() {
            return (node_to_expand.id(), new_state);
        }
        let unvisited_moves = &mut node_to_expand.value().unvisited_moves;
        let index = thread_rng().gen_range(0, unvisited_moves.len());
        let move_to_expand = unvisited_moves[index].clone();
        unvisited_moves[index] = unvisited_moves.last().unwrap().clone();
        unvisited_moves.pop();

        let mut new_historic = node_to_expand.value().state.clone();
        new_state.do_move(&move_to_expand);
        new_historic.push(move_to_expand);

        let new_node = MctsNode {
            sum_rewards: num_traits::zero(),
            n_visits: 0,
            unvisited_moves: new_state.legals_moves(),
            hash: new_state.hash(),
            state: new_historic,
            additional_info: Default::default(),
        };

        (node_to_expand.append(new_node).id(), new_state)
    }
}

impl<State: GameTrait, EV: Evaluator<State, A, Args=u32>, A: Clone + Default>
LazyTreePolicy<State, EV, A> for DefaultLazyTreePolicy<State, EV, A>
{
    fn tree_policy(
        tree: &mut LazyMctsTree<State, EV::Reward, A>,
        root_state: State,
    ) -> (NodeId, State) {
        let master_player = root_state.player_turn();
        let selected_node_id = Self::select(tree, &master_player);
        Self::expand(tree.get_mut(selected_node_id).unwrap(), root_state)
    }

    fn update_state(mut root_state: State, historic: &[State::Move]) -> State {
        for m in historic {
            root_state.do_move(m)
        }
        root_state
    }

    fn best_child(
        tree: &LazyMctsTree<State, EV::Reward, A>,
        turn: &State::Player,
        parent_id: NodeId,
    ) -> NodeId {
        let parent_node = tree.get(parent_id).unwrap();
        let parent_visits = parent_node.value().n_visits;
        parent_node
            .children()
            .max_by_key(|child| EV::eval_child(&child.value(), turn, &parent_visits))
            .unwrap()
            .id()
    }
}

/// Uses UCT to evaluate nodes, and evaluates an endstate with 1 if the player won.
pub struct DefaultUctEvaluator {}

impl<State: GameTrait, AdditionalInfo: Clone + Default> Evaluator<State, AdditionalInfo>
for DefaultUctEvaluator
{
    type Args = u32;
    type Reward = u32;

    fn eval_child(
        child: &LazyMctsNode<State, Self::Reward, AdditionalInfo>,
        _turn: &State::Player,
        parent_visits: &Self::Args,
    ) -> Num {
        uct_value(
            *parent_visits,
            n64(child.sum_rewards as f64),
            child.n_visits,
        )
    }

    fn evaluate_leaf(child: State, turn: &State::Player) -> u32 {
        if child.get_winner() == *turn {
            1
        } else {
            0
        }
    }
}
