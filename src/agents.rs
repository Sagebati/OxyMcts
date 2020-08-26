use crate::{DefaultMcts, GameTrait};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn mcts_agent<Game: GameTrait>(state: &Game, playouts: usize) -> Game::Move {
    let mut mcts = DefaultMcts::new(state);
    for _ in 0..playouts {
        mcts.execute(());
    }
    mcts.best_move()
}

pub fn random_agent<Game: GameTrait>(state: &Game) -> Game::Move {
    state.legals_moves().choose(&mut thread_rng()).unwrap().clone()
}
