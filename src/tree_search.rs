use core::fmt;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use num_traits::Zero;

use crate::alisases::{LazyMctsNode, LazyMctsTree};
use crate::Evaluator;
use crate::traits::{BackPropPolicy, GameTrait, LazyTreePolicy, Playout};

/// This is a special MCTS because it doesn't store the state in the node but instead stores the
/// historic to the node.
#[derive(Clone)]
pub struct LazyMcts<
    State: GameTrait,
    TP,
    PP,
    BP,
    EV: Evaluator<State, AddInfo>,
    AddInfo: Clone + Default,
> where
    PP: Playout<State>,
    TP: LazyTreePolicy<State, EV, AddInfo>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, EV::Reward, AddInfo>,
{
    root_state: State,
    tree_policy: PhantomData<TP>,
    playout_policy: PhantomData<PP>,
    backprop_policy: PhantomData<BP>,
    evaluator: PhantomData<EV>,
    tree: LazyMctsTree<State, EV::Reward, AddInfo>,
}

impl<
    State: GameTrait,
    TP: LazyTreePolicy<State, EV, A>,
    PP,
    BP,
    EV,
    A: Clone + Default,
> LazyMcts<State, TP, PP, BP, EV, A>
    where
        PP: Playout<State>,
        BP: BackPropPolicy<Vec<State::Move>, State::Move, EV::Reward, A>,
        EV: Evaluator<State, A>,
{
    pub fn new(root_state: State) -> Self {
        Self::with_capacity(root_state, 0)
    }

    pub fn with_capacity(root_state: State, capacity: usize) -> Self {
        let tree = LazyMctsTree::<State, EV::Reward, A>::with_capacity(
            LazyMctsNode::<State, EV::Reward, A> {
                sum_rewards: Zero::zero(),
                n_visits: 0,
                unvisited_moves: root_state.legals_moves(),
                hash: root_state.hash(),
                state: vec![],
                additional_info: Default::default(),
            },
            capacity,
        );
        Self {
            root_state,
            tree_policy: PhantomData,
            playout_policy: PhantomData,
            backprop_policy: PhantomData,
            evaluator: PhantomData,
            tree,
        }
    }

    /// Executes one selection, expansion?, simulation, backpropagation.
    pub fn execute(&mut self, playout_args: PP::Args) {
        let (node_id, state) = TP::tree_policy(&mut self.tree, self.root_state.clone());
        let final_state = PP::playout(state, playout_args);
        let eval = EV::evaluate_leaf(final_state, &self.root_state.player_turn());
        BP::backprop(&mut self.tree, node_id, eval);
    }

    /// Returns the best move from the root.
    pub fn best_move(&self) -> State::Move {
        let best_child = TP::best_child(
            &self.tree,
            &self.root_state.player_turn(),
            self.tree.root().id(),
        );
        self.tree
            .get(best_child)
            .unwrap()
            .value()
            .state
            .last()
            .expect("The historic of the children of the root is empty, cannot happen")
            .clone()
    }
}

impl<
    State: GameTrait,
    TP: LazyTreePolicy<State, EV, A>,
    PP,
    BP,
    EV,
    A: Clone + Default + Debug,
> Debug for LazyMcts<State, TP, PP, BP, EV, A>
    where
        PP: Playout<State>,
        BP: BackPropPolicy<Vec<State::Move>, State::Move, EV::Reward, A>,
        EV: Evaluator<State, A>,
        EV::Reward: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self.tree))
    }
}