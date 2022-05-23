use crate::logic::models::Coordinate;
use crate::Unit;
use std::collections::HashMap;

pub enum OpponentType {
    TheSimpleton,
}

pub struct OpponentMove {
    pub piece: Unit,
    pub move_to: Coordinate,
}

pub trait Opponent {
    fn get_opposing_move(
        &self,
        active_pieces: &HashMap<String, Unit>,
        moved_piece: &Unit,
    ) -> OpponentMove;
}
