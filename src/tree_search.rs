use std::marker::PhantomData;

use num_traits::Zero;

use crate::alisases::{LazyMctsNode, LazyMctsTree};
use crate::traits::{BackPropPolicy, Evaluator, GameTrait, LazyTreePolicy, Playout};

/// This is a special MCTS because it doesn't store the state in the node but instead stores the
/// historic to the node.
pub struct LazyMcts<TP, PP, BP, EV, AddInfo: Clone + Default>
    where
        PP: Playout<EV::State>,
        TP: LazyTreePolicy<EV, AddInfo>,
        BP: BackPropPolicy<
            Vec<<EV::State as GameTrait>::Move>,
            <EV::State as GameTrait>::Move,
            <EV::State as GameTrait>::Reward,
            AddInfo,
            EV::LeafEval,
        >,
        EV: Evaluator<AddInfo>,
{
    root_state: EV::State,
    tree_policy: PhantomData<TP>,
    playout_policy: PhantomData<PP>,
    backprop_policy: PhantomData<BP>,
    evaluator: PhantomData<EV>,
    tree: LazyMctsTree<EV::State, AddInfo>,
}

impl<TP: LazyTreePolicy<EV, A>, PP, BP, EV, A: Clone + Default> LazyMcts<TP, PP, BP, EV, A>
    where
        PP: Playout<EV::State>,
        BP: BackPropPolicy<
            Vec<<EV::State as GameTrait>::Move>,
            <EV::State as GameTrait>::Move,
            <EV::State as GameTrait>::Reward,
            A,
            EV::LeafEval,
        >,
        EV: Evaluator<A>,
{
    pub fn new(root_state: EV::State) -> Self {
        Self::with_capacity(root_state, 0)
    }

    pub fn with_capacity(root_state: EV::State, capacity: usize) -> Self {
        let tree = LazyMctsTree::<EV::State, A>::with_capacity(
            LazyMctsNode::<EV::State, A> {
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

    pub fn execute(&mut self, playout_args: PP::Args) {
        let (node_id, state) = TP::tree_policy(&mut self.tree, self.root_state.clone());
        let final_state = PP::playout(state, playout_args);
        let eval = EV::evaluate_leaf(&final_state, &self.root_state.player_turn());
        BP::backprop(&mut self.tree, node_id, eval);
    }

    pub fn best_move(&self) -> <EV::State as GameTrait>::Move {
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
            .expect("The historic of the children of the root is empty, cannot happend")
            .clone()
    }
}
