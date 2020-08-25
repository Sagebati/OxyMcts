use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div};

use ego_tree::{NodeId, Tree, NodeMut};
use num_traits::{ToPrimitive, Zero};
use rand::prelude::{IteratorRandom};
use rand::{thread_rng, Rng};

use crate::mcts_node::MctsNode;
use crate::traits::{BackPropPolicy, GameTrait, Playout, Evaluator, LazyTreePolicy};
use crate::alisases::{LazyMctsTree, LazyMctsNode};

pub struct DefaultBackProp {}

impl<T: Clone,
    Move: Clone,
    R: Add + AddAssign + Div + Clone + Zero + ToPrimitive,
    A: Clone + Default,
> BackPropPolicy<T, Move, R, A, R> for DefaultBackProp {
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

pub struct DefaultPlayout<T: GameTrait> {
    marker: PhantomData<T>,
}

impl<T: GameTrait> Playout<T> for DefaultPlayout<T> {
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

pub struct DefaultLazyTreePolicy<EV: Evaluator<A>, A: Clone + Default> {
    phantom_a: PhantomData<A>,
    phantom_ev: PhantomData<EV>,
}

impl<EV: Evaluator<A, Args=u32>, A: Clone + Default> DefaultLazyTreePolicy<EV, A> {
    pub fn select(
        mut tree: &mut LazyMctsTree<EV::State, A>,
        turn: &<EV::State as GameTrait>::Player,
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
        mut node_to_expand: NodeMut<LazyMctsNode<EV::State, A>>,
        root_state: EV::State,
    ) -> (NodeId, EV::State) {
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

impl<EV: Evaluator<A, Args=u32>, A: Clone + Default> LazyTreePolicy<EV, A>
for DefaultLazyTreePolicy<EV, A>
{
    fn tree_policy(
        tree: &mut LazyMctsTree<<EV as Evaluator<A>>::State, A>,
        root_state: <EV as Evaluator<A>>::State,
    ) -> (NodeId, <EV as Evaluator<A>>::State) {
        let master_player = root_state.player_turn();
        let selected_node_id = Self::select(tree, &master_player);
        Self::expand(tree.get_mut(selected_node_id).unwrap(), root_state)
    }

    fn update_state(
        mut root_state: <EV as Evaluator<A>>::State,
        historic: &[<<EV as Evaluator<A>>::State as GameTrait>::Move],
    ) -> <EV as Evaluator<A>>::State {
        for m in historic {
            root_state.do_move(m)
        }
        root_state
    }

    fn best_child(
        tree: &LazyMctsTree<<EV as Evaluator<A>>::State, A>,
        turn: &<<EV as Evaluator<A>>::State as GameTrait>::Player,
        parent_id: NodeId,
    ) -> NodeId {
        let parent_node = tree.get(parent_id).unwrap();
        let parent_visits = parent_node.value().n_visits;
        parent_node.children()
            .max_by_key(|child| EV::eval_child(&child.value(), turn, &parent_visits))
            .unwrap()
            .id()
    }
}
