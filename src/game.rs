#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::VecDeque;

pub struct GameState {
    
}

pub enum GameStage {
    BetStage,
    GameStage
}

pub struct BetHistory {
    // history: VecDeque<>
}


pub trait Player {
}

pub struct Game {
    stage: GameStage
}

impl Game {
}

pub mod history {

}

