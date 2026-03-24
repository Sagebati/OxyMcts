use core::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::fs::metadata;
use std::ops::{Add, Div};

use ascii_tree::Tree::{Leaf, Node};
use ascii_tree::{write_tree, Tree};
use bon::{bon, builder, Builder};
use ego_tree::NodeId;
use num_traits::{ToPrimitive, Zero};

use crate::aliases::LazyMctsTree;
use crate::traits::{BackPropPolicy, GameTrait, LazyTreePolicy, Playout};
use crate::tree_search::lazy_mcts_builder::State;
use crate::{Evaluator, LazyMctsNode};

fn create_tree<State, Reward, Metadata>(
    root: State,
    metadata: Metadata,
) -> LazyMctsTree<State, Reward, Metadata>
where
    State: GameTrait,
    Reward: Clone + Default,
    Metadata: Clone + Default,
{
}

/// This is a special MCTS because it doesn't store the state in the node but instead stores the
/// historic to the node.
pub struct LazyMcts<'a, State, TP, PP, BP, EV, Metadata, Reward>
where
    State: GameTrait,
    TP: LazyTreePolicy<State, EV, Metadata, Reward>,
    PP: Playout<State>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, Reward, Metadata, EV::EvalResult>,
    EV: Evaluator<State, Reward, Metadata>,
    Metadata: Clone + Default,
    Reward: Clone,
{
    root_state: &'a State,
    tree_policy: TP,
    playout_policy: PP,
    backprop_policy: BP,
    evaluator: EV,
    tree: LazyMctsTree<State, Reward, Metadata>,
}

#[bon]
impl<'a, State, TP, PP, BP, EV, A, R> LazyMcts<'a, State, TP, PP, BP, EV, A, R>
where
    State: GameTrait,
    TP: LazyTreePolicy<State, EV, A, R>,
    PP: Playout<State>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, R, A, EV::EvalResult>,
    EV: Evaluator<State, R, A>,
    A: Clone + Default,
    R: Clone + Div + ToPrimitive + Zero + Add + Display,
{
    #[builder]
    pub fn new(
        state: &'a State,
        #[builder(default = crate::defaults::DefaultLazyTreePolicy::default())] tree_policy: TP,
        #[builder(default = crate::defaults::RandomPlayout)] playout_policy: PP,
        #[builder(default =  crate::defaults::DefaultBackProp)] backprop_policy: BP,
        #[builder(default = crate::defaults::DefaultUctEvaluator)] evaluator: EV,
        metadata: A,
        capacity: Option<usize>,
    ) -> Self {
        let tree = ego_tree::Tree::new(
            LazyMctsNode::builder()
                .state(state)
                .metadata(metadata)
                .build(),
        );
        Self {
            root_state: &state,
            tree_policy,
            playout_policy,
            backprop_policy,
            evaluator,
            tree,
        }
    }

    /// Executes one selection, expansion?, simulation, backpropagation.
    pub fn execute(&mut self, evaluation_args: &EV::Args, playout_args: PP::Args) {
        let (node_id, state) =
            TP::tree_policy(&mut self.tree, self.root_state.clone(), evaluation_args);
        let final_state = self.playout_policy.playout(state, playout_args);
        let eval = EV::evaluate_leaf(final_state, &self.root_state.player_turn());
        BP::backprop(&mut self.tree, node_id, eval);
    }

    /// Returns the best move from the root.
    pub fn best_move(&self, evaluator_args: &EV::Args) -> State::Move {
        let best_child = TP::best_child(
            &self.tree,
            &self.root_state.player_turn(),
            self.tree.root().id(),
            evaluator_args,
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

    pub fn write_tree(&self) -> String {
        let tree = self.dfs(self.tree.root().id());
        let mut output = String::new();
        write_tree(&mut output, &tree).unwrap();
        output
    }

    fn dfs(&self, node_id: NodeId) -> Tree {
        let node = self.tree.get(node_id).unwrap();
        if node.has_children() {
            let mut nodes = vec![];
            for c in node.children() {
                nodes.push(self.dfs(c.id()))
            }
            Node(
                format!("{};{}", node.value().n_visits, node.value().sum_rewards),
                nodes,
            )
        } else {
            Leaf(vec![format!(
                "{};{}",
                node.value().n_visits,
                node.value().sum_rewards
            )])
        }
    }

    pub fn tree(&self) -> &LazyMctsTree<State, R, A> {
        &self.tree
    }
}

impl<State, TP, PP, BP, EV, A, R> Debug for LazyMcts<'_, State, TP, PP, BP, EV, A, R>
where
    State: GameTrait,
    TP: LazyTreePolicy<State, EV, A, R>,
    PP: Playout<State>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, R, A, EV::EvalResult>,
    EV: Evaluator<State, R, A>,
    EV::EvalResult: Debug,
    A: Clone + Default + Debug,
    R: Clone + Debug + Div + Add + Zero + ToPrimitive,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self.tree))
    }
}
