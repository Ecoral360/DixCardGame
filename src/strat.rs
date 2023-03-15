use crate::{game::{BetHistory, GameState}, cards::Card};

pub trait BetStrategy {
    fn bet(&mut self, bet_history: BetHistory) -> Option<u8>;
}

pub trait PlayStrategy {
    fn play(&mut self, game_state: GameState) -> Card;
}

