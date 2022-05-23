use crate::logic::models::{InternalBoard, Unit};
use crate::logic::movement::make_move::perform_move_and_get_active_pieces;
use crate::logic::movement::move_validation::validate_move;
use crate::logic::opponent::factory::get_opponent;
use crate::logic::opponent::types::{Opponent, OpponentType};
use std::collections::HashMap;

fn do_opposing_move(
    active_pieces: HashMap<String, Unit>,
    moved_piece: Unit,
) -> HashMap<String, Unit> {
    let opponent = get_opponent(OpponentType::TheSimpleton);

    let opponent_move = opponent.get_opposing_move(&active_pieces, &moved_piece);

    return perform_move_and_get_active_pieces(
        active_pieces,
        opponent_move.piece,
        opponent_move.move_to.x,
        opponent_move.move_to.y,
    );
}

pub fn do_move(board: InternalBoard, unit: Unit, to_x: i8, to_y: i8) -> InternalBoard {
    let active_pieces = perform_move_and_get_active_pieces(board.active_pieces, unit, to_x, to_y);

    InternalBoard {
        active_pieces: do_opposing_move(active_pieces, unit),
        human_player: board.human_player,
    }
}

pub fn is_valid_board_move(board: &InternalBoard, unit: &Unit, to_x: i8, to_y: i8) -> bool {
    validate_move(&board.active_pieces, unit, to_x, to_y)
}
