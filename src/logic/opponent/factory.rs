use crate::logic::opponent::the_simpleton::TheSimpleton;
use crate::logic::opponent::types::{Opponent, OpponentType};

pub fn get_opponent(opponent_type: OpponentType) -> impl Opponent {
    match opponent_type {
        OpponentType::TheSimpleton => TheSimpleton {},
    }
}
