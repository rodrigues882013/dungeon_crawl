#![warn(clippy::pedantic)]
use super::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
