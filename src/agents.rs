use crate::{DefaultMcts, GameTrait};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn mcts_uct_agent<State: GameTrait>(state: &State, playouts: usize, c: f64) -> State::Move {
    let mut mcts = DefaultMcts::new(state);
    for _ in 0..playouts {
        mcts.execute(&c, ());
    }
    mcts.best_move(&c)
}

pub fn random_agent<State: GameTrait>(state: &State) -> State::Move {
    state
        .legals_moves()
        .choose(&mut thread_rng())
        .unwrap()
        .clone()
}
