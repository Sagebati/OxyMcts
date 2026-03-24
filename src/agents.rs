use rand::prelude::{IndexedRandom, SliceRandom, ThreadRng};

use crate::defaults::{DefaultUctEvaluator, RandomPlayout};
use crate::{DefaultMcts, GameTrait};

/// Creates an MCTS tree from the state and returns the best move.
pub fn mcts_uct_agent<Game: GameTrait>(state: &Game, playouts: usize, c: f64) -> Game::Move {
    let mut mcts = DefaultMcts::builder()
        .state(state)
        .playout_policy(RandomPlayout)
        .metadata(())
        .build();
    for _ in 0..playouts {
        mcts.execute(&c, ());
    }
    mcts.best_move(&c)
}

/// Returns a random move.
pub fn random_agent<Game: GameTrait>(state: &Game, thread_rng: &mut ThreadRng) -> Game::Move {
    state.legals_moves().choose(thread_rng).unwrap().clone()
}
