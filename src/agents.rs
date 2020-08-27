use crate::{DefaultMcts, GameTrait};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn mcts_uct_agent<Game: GameTrait>(state: &Game, playouts: usize, c : f64) -> Game::Move {
    let mut mcts = DefaultMcts::new(state);
    for _ in 0..playouts {
        mcts.execute(&c, ());
    }
    mcts.best_move(&c)
}

pub fn random_agent<Game: GameTrait>(state: &Game) -> Game::Move {
    state.legals_moves().choose(&mut thread_rng()).unwrap().clone()
}
