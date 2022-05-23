use crate::logic::opponent::types::{Opponent, OpponentType};

use crate::logic::opponent::the_aggressive_simpleton::TheAggressiveSimpleton;
use crate::logic::opponent::the_simpleton::TheSimpleton;

pub fn get_opponent(opponent_type: OpponentType) -> Box<dyn Opponent> {
    match opponent_type {
        OpponentType::TheSimpleton => Box::new(TheSimpleton {}),
        OpponentType::TheAggressiveSimpleton => Box::new(TheAggressiveSimpleton {}),
    }
}
