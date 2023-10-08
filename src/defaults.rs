use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div};

use ego_tree::{NodeId, NodeMut, Tree};
use noisy_float::types::n64;
use num_traits::{ToPrimitive, Zero};
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;

use crate::{Evaluator, Nat, Num, uct_value};
use crate::aliases::{LazyMctsNode, LazyMctsTree};
use crate::mcts_node::MctsNode;
use crate::traits::{BackPropPolicy, GameTrait, LazyTreePolicy, Playout};

/// A default backprop policy it will take the reward of the simulation and backkpropagate the
/// result  to the branch nodes.
pub struct DefaultBackProp;

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
pub struct DefaultPlayout;

impl<T: GameTrait> Playout<T> for DefaultPlayout {
    type Args = ();

    fn playout(mut state: T, _args: ()) -> T {
        while !state.is_final() {
            let moves = state
                .legals_moves();
            let m = moves.choose(&mut thread_rng()) .unwrap();
            state.do_move(m);
        }
        state
    }
}

/// Explores at least once each child node, before going deeper.
pub struct DefaultLazyTreePolicy<State: GameTrait, EV: Evaluator<State, Reward, A>, A: Clone +
Default, Reward: Clone> {
    phantom_state: PhantomData<State>,
    phantom_a: PhantomData<A>,
    phantom_ev: PhantomData<EV>,
    phamtom_r: PhantomData<Reward>,
}

impl<State: GameTrait, EV: Evaluator<State, Reward, A, Args=f64>, A: Clone + Default,
    Reward: Clone>
DefaultLazyTreePolicy<State, EV, A, Reward>
    where
        Reward: Div + ToPrimitive + Add + Zero,
{
    pub fn select(
        tree: &mut LazyMctsTree<State, Reward, A>,
        turn: &State::Player,
        evaluator_args: &EV::Args,
    ) -> NodeId {
        let mut current_node_id = tree.root().id();
        while tree.get(current_node_id).unwrap().has_children() {
            if tree.get(current_node_id).unwrap().value().can_add_child() {
                return current_node_id;
            } else {
                current_node_id =
                    Self::best_child(tree, turn, current_node_id, evaluator_args);
            }
        }
        current_node_id
    }

    pub fn expand(
        mut node_to_expand: NodeMut<LazyMctsNode<State, Reward, A>>,
        root_state: State,
    ) -> (NodeId, State) {
        let mut new_state = Self::update_state(root_state, &node_to_expand.value().state);
        if !node_to_expand.value().can_add_child() {
            return (node_to_expand.id(), new_state);
        }
        let unvisited_moves = &mut node_to_expand.value().unvisited_moves;
        let index = thread_rng().gen_range(0..unvisited_moves.len());
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

impl<State, EV, A, Reward>
LazyTreePolicy<State, EV, A, Reward> for DefaultLazyTreePolicy<State, EV, A, Reward>
    where
        State: GameTrait,
        Reward: Clone + Div + Add + ToPrimitive + Zero,
        EV: Evaluator<State, Reward, A, Args=f64>,
        A: Clone + Default
{
    fn tree_policy(
        tree: &mut LazyMctsTree<State, Reward, A>,
        root_state: State,
        evaluator_args: &EV::Args,
    ) -> (NodeId, State) {
        let master_player = root_state.player_turn();
        let selected_node_id = Self::select(tree, &master_player, evaluator_args);
        let node = tree
            .get_mut(selected_node_id)
            .unwrap();
        Self::expand(node, root_state)
    }

    fn update_state(mut root_state: State, historic: &[State::Move]) -> State {
        for m in historic {
            root_state.do_move(m)
        }
        root_state
    }

    fn best_child(
        tree: &LazyMctsTree<State, Reward, A>,
        turn: &State::Player,
        parent_id: NodeId,
        eval_args: &EV::Args,
    ) -> NodeId {
        let parent_node = tree.get(parent_id).unwrap();
        let n_visits = parent_node.value().n_visits;
        parent_node
            .children()
            .max_by_key(|child| EV::eval_child(child.value(), turn, n_visits, eval_args))
            .unwrap()
            .id()
    }
}

/// Uses UCT to evaluate nodes, and evaluates an end state with 1 if the player won.
pub struct DefaultUctEvaluator;

impl<State: GameTrait, AdditionalInfo: Clone + Default, Reward: Clone + Div + Zero + ToPrimitive
+ Add>
Evaluator<State, Reward, AdditionalInfo>
for DefaultUctEvaluator
{
    type Args = f64;
    type EvalResult = Nat;

    fn eval_child(
        child: &LazyMctsNode<State, Reward, AdditionalInfo>,
        _turn: &State::Player,
        parent_visits: Nat,
        &c: &Self::Args,
    ) -> Num {
        if child.n_visits == 0 {
            return n64(0f64);
        }
        uct_value(
            parent_visits,
            child.sum_rewards.to_f64().unwrap(),
            child.n_visits,
            c,
        )
    }

    fn evaluate_leaf(child: State, turn: &State::Player) -> Self::EvalResult {
        if child.get_winner() == *turn {
            1
        } else {
            0
        }
    }
}
