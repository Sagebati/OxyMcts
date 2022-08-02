use rand::prelude::{SliceRandom, ThreadRng};

use crate::{DefaultMcts, GameTrait};

pub fn mcts_uct_agent<Game: GameTrait>(state: &Game, playouts: usize, c: f64) -> Game::Move {
    let mut mcts = DefaultMcts::new(state);
    for _ in 0..playouts {
        mcts.execute(&c, ());
    }
    mcts.best_move(&c)
}

pub fn random_agent<Game: GameTrait>(state: &Game, thread_rng: &mut ThreadRng) -> Game::Move {
    state
        .legals_moves()
        .choose(thread_rng)
        .unwrap()
        .clone()
}
