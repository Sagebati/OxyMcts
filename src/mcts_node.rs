use std::fmt::{Debug, Error, Formatter};
use std::ops::{Add, Deref, DerefMut, Div};

use num_traits::{ToPrimitive, Zero};

use crate::alisases::Nat;
use crate::traits::GameTrait;

#[derive(Clone)]
pub struct MctsNode<T, Move, Reward, AdditionalInfo = ()>
    where
        Reward: Clone + Add + Div + Zero + ToPrimitive,
        T: Clone,
        Move: Clone,
        AdditionalInfo: Clone + Default,
{
    pub sum_rewards: Reward,
    pub n_visits: Nat,
    /// All the moves who don't have a node. at the creation this list contains all the legals
    /// moves from the state.
    pub unvisited_moves: Vec<Move>,
    pub hash: u64,
    pub state: T,
    pub additional_info: AdditionalInfo,
}

impl<T, Move, Reward, AdditionalInfo> MctsNode<T, Move, Reward, AdditionalInfo>
    where
        Reward: Clone + Add + Div + Zero + ToPrimitive,
        T: Clone,
        Move: Clone,
        AdditionalInfo: Clone + Default,
{
    #[inline]
    pub fn can_add_child(&self) -> bool {
        !self.unvisited_moves.is_empty()
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl<T, M, R, A> Deref for MctsNode<T, M, R, A>
    where
        R: Clone + Add + Div + ToPrimitive + Zero,
        T: Clone + GameTrait,
        M: Clone,
        A: Clone + Default,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T, M, R, A> DerefMut for MctsNode<T, M, R, A>
    where
        R: Clone + Add + Div + Zero + ToPrimitive,
        T: Clone + GameTrait,
        M: Clone,
        A: Clone + Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<T, M, R, A> Debug for MctsNode<T, M, R, A>
    where
        R: Clone + Add + Div + ToPrimitive + Zero,
        T: Clone,
        M: Clone,
        A: Clone + Default,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("MctsNode")
            .field("n_visits", &self.n_visits)
            .field("unvisited_moves_len", &self.unvisited_moves.len())
            .finish()
    }
}

/// Unstable
impl<T, M, R, A> PartialEq for MctsNode<T, M, R, A>
    where
        R: Clone + Add + Div + Zero + ToPrimitive,
        T: Clone,
        M: Clone,
        A: Clone + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
