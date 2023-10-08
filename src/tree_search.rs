use core::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, Div};
use std::sync::Mutex;

use ascii_tree::Tree::{Leaf, Node};
use ascii_tree::{write_tree, Tree};
use ego_tree::NodeId;
use num_traits::{ToPrimitive, Zero};

use crate::aliases::{LazyMctsNode, LazyMctsTree};
use crate::traits::{BackPropPolicy, GameTrait, LazyTreePolicy, Playout};
use crate::Evaluator;

/// This is a special MCTS because it doesn't store the state in the node but instead stores the
/// historic to the node.

pub struct LazyMcts<'a, State, TP, PP, BP, EV, AddInfo, Reward>
where
    State: GameTrait,
    TP: LazyTreePolicy<State, EV, AddInfo, Reward>,
    PP: Playout<State>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, Reward, AddInfo, EV::EvalResult>,
    EV: Evaluator<State, Reward, AddInfo>,
    AddInfo: Clone + Default,
    Reward: Clone,
{
    root_state: &'a State,
    tree_policy: PhantomData<TP>,
    playout_policy: PhantomData<PP>,
    backprop_policy: PhantomData<BP>,
    evaluator: PhantomData<EV>,
    tree: Mutex<LazyMctsTree<State, Reward, AddInfo>>,
}

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
    pub fn new(root_state: &'a State) -> Self {
        Self::with_capacity(root_state, 0)
    }

    pub fn with_capacity(root_state: &'a State, capacity: usize) -> Self {
        let tree = LazyMctsTree::<State, R, A>::with_capacity(
            LazyMctsNode::<State, R, A> {
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
            tree: Mutex::new(tree),
        }
    }

    /// Executes one selection, expansion?, simulation, backpropagation.
    pub fn execute(&self, evaluation_args: &EV::Args, playout_args: PP::Args) {
        let mut tree = self.tree.lock().unwrap();
        let (node_id, state) = TP::tree_policy(&mut tree, self.root_state.clone(), evaluation_args);
        drop(tree);

        let final_state = PP::playout(state, playout_args);
        let eval = EV::evaluate_leaf(final_state, &self.root_state.player_turn());

        let mut tree = self.tree.lock().unwrap();
        BP::backprop(&mut tree, node_id, eval);
    }

    /// Returns the best move from the root.
    pub fn best_move(&self, evaluator_args: &EV::Args) -> State::Move {
        let tree = self.tree.lock().unwrap();
        let best_child = TP::best_child(
            &tree,
            &self.root_state.player_turn(),
            tree.root().id(),
            evaluator_args,
        );
        tree.get(best_child)
            .unwrap()
            .value()
            .state
            .last()
            .expect("The historic of the children of the root is empty, cannot happen")
            .clone()
    }

    pub fn write_tree(&self) -> String {
        let tree = self.dfs(self.tree.lock().unwrap().root().id());
        let mut output = String::new();
        write_tree(&mut output, &tree).unwrap();
        output
    }

    fn dfs(&self, node_id: NodeId) -> Tree {
        let tree = self.tree.lock().unwrap();
        let node = tree.get(node_id).unwrap();
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

impl<State, TP, PP, BP, EV, A, R> Clone for LazyMcts<'_, State, TP, PP, BP, EV, A, R>
where
    State: GameTrait,
    TP: LazyTreePolicy<State, EV, A, R>,
    PP: Playout<State>,
    BP: BackPropPolicy<Vec<State::Move>, State::Move, R, A, EV::EvalResult>,
    EV: Evaluator<State, R, A>,
    A: Clone + Default,
    R: Clone + Debug + Div + Add + Zero + ToPrimitive,
{
    fn clone(&self) -> Self {
        Self {
            root_state: self.root_state,
            tree_policy: PhantomData,
            playout_policy: PhantomData,
            backprop_policy: PhantomData,
            evaluator: PhantomData,
            tree: Mutex::new(self.tree.lock().unwrap().clone()),
        }
    }
}
