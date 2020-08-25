use lib_mcts::{GameTrait, Evaluator, LazyMctsNode, Num, LazyMctsTree, DefaultMcts, uct_value};
use std::collections::{HashSet};
use noisy_float::prelude::{n64};


#[derive(Debug, Clone, Default)]
struct TicTacToe {
    /// true cross, false circle
    turn: bool,
    grid: Vec<Vec<u8>>,
    sums_cols: Vec<(usize, usize)>,
    sums_diags: [(usize, usize); 2],
    sums_rows: Vec<(usize, usize)>,
    coordinates_2nd_diag: HashSet<(usize, usize)>,
    n: usize,
}

impl TicTacToe {
    fn new(n: usize) -> Self {
        let mut coordinates_2nd_diag = vec![(n - 1, 0)];
        for i in 0..=n - 2 {
            let (x, y) = coordinates_2nd_diag[i];
            coordinates_2nd_diag.push((x - 1, y + 1))
        }
        let coordinates_2nd_diag = coordinates_2nd_diag.into_iter().collect();
        TicTacToe {
            turn: true,
            grid: vec![vec![0; n]; n],
            sums_cols: vec![(0, 0); n],
            sums_diags: [(0, 0); 2],
            sums_rows: vec![(0, 0); n],
            coordinates_2nd_diag,
            n,
        }
    }

    pub fn get_turn(&self) -> bool {
        self.turn
    }

    pub fn legal_moves(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let l = self.grid.len();
        for i in 0..l {
            for j in 0..l {
                if self.grid[i][j] == 0 {
                    res.push((i, j))
                }
            }
        }
        res
    }

    pub fn play(&mut self, p: (usize, usize)) {
        self.grid[p.0][p.1] = if self.turn {
            self.sums_cols[p.0].0 += 1;
            self.sums_rows[p.1].0 += 1;
            if p.0 == p.1 {
                self.sums_diags[0].0 += 1;
            }
            if self.coordinates_2nd_diag.contains(&p) {
                self.sums_diags[1].0 += 1;
            }
            1
        } else {
            self.sums_cols[p.0].1 += 1;
            self.sums_rows[p.1].1 += 1;
            if p.0 == p.1 {
                self.sums_diags[0].1 += 1;
            }
            if self.coordinates_2nd_diag.contains(&p) {
                self.sums_diags[1].1 += 1;
            }
            2
        };
    }

    // Return None is the game is not finished, if not return the winner
    pub fn finished(&self) -> Option<bool> {
        self.sums_cols.iter()
            .chain(self.sums_diags.iter())
            .chain(self.sums_rows.iter())
            .find(move |x| x.0 == self.n || x.1 == self.n)
            .map(move |x| match x {
                (y, _) if self.n == *y => true,
                (_, y) if self.n == *y => false,
                _ => unreachable!()
            })
    }
}

impl GameTrait for TicTacToe {
    type Player = bool;
    type Move = (usize, usize);
    type Reward = i32;

    fn legals_moves(&self) -> Vec<Self::Move> {
        self.legal_moves()
    }

    fn player_turn(&self) -> Self::Player {
        self.get_turn()
    }

    fn hash(&self) -> u64 {
        0
    }

    fn is_final(&self) -> bool {
        self.finished().is_some()
    }

    fn do_move(&mut self, m: &Self::Move) {
        self.play(*m);
    }
}


struct E {}

impl Evaluator<()> for E {
    type State = TicTacToe;
    type LeafEval = i32;
    type Args = u32;

    fn eval_child(child: &LazyMctsNode<Self::State, ()>, _turn: &bool, parent_visit: &Self::Args) -> Num {
        uct_value(*parent_visit, n64(child.sum_rewards as f64), child.n_visits)
    }

    fn evaluate_leaf(child: &Self::State, turn: &bool) -> Self::LeafEval {
        if child.finished().unwrap() == *turn {
            1
        } else {
            0
        }
    }
}

fn main() {
    let mut mcts = DefaultMcts::<E>::new(TicTacToe::new(5)); for _ in 0..100000 {
        mcts.execute(());
    }
    dbg!(mcts.best_move());
}