use std::marker::PhantomData;

use num_traits::Zero;

use crate::alisases::{LazyMctsNode, LazyMctsTree};
use crate::traits::{BackPropPolicy, GameTrait, LazyTreePolicy, Playout};
use crate::EvaluatorBis;

/// This is a special MCTS because it doesn't store the state in the node but instead stores the
/// historic to the node.
pub struct LazyMcts<
    State: GameTrait,
    TP,
    PP,
    BP,
    EV: EvaluatorBis<State, AddInfo>,
    AddInfo: Clone + Default,
    SimulationResult = (),
> where
    PP: Playout<State>,
    TP: LazyTreePolicy<State, EV, AddInfo>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, EV::Reward, AddInfo, EV::Reward>,
{
    root_state: State,
    tree_policy: PhantomData<TP>,
    playout_policy: PhantomData<PP>,
    backprop_policy: PhantomData<BP>,
    // Cannot put EV::REward in default parameter for SimulationResult.
    simulation_t: PhantomData<SimulationResult>,
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
        PlayoutResult,
    > LazyMcts<State, TP, PP, BP, EV, A, PlayoutResult>
where
    PP: Playout<State>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, EV::Reward, A, EV::Reward>,
    EV: EvaluatorBis<State, A>,
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
            simulation_t: PhantomData,
            tree,
        }
    }

    pub fn execute(&mut self, playout_args: PP::Args) {
        let (node_id, state) = TP::tree_policy(&mut self.tree, self.root_state.clone());
        let final_state = PP::playout(state, playout_args);
        let eval = EV::evaluate_leaf(final_state, &self.root_state.player_turn());
        BP::backprop(&mut self.tree, node_id, eval);
    }

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
