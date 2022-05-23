use crate::logic::movement::utilities::get_piece_being_jumped;
use std::collections::HashMap;
use web_sys::console;

use crate::logic::models::get_key_for_unit;
use crate::Unit;

pub fn perform_move_and_get_active_pieces(
    active_pieces: HashMap<String, Unit>,
    unit: Unit,
    to_x: i8,
    to_y: i8,
) -> HashMap<String, Unit> {
    let mut modified_active_pieces = active_pieces;

    let unit_to_move = modified_active_pieces.remove(&get_key_for_unit(&unit));

    if let Some(mut unit_to_move) = unit_to_move {
        console::log_1(&"Moving unit".into());

        if let Some(active_piece_being_jumped) =
            get_piece_being_jumped(&(modified_active_pieces.clone()), &unit, to_x, to_y)
        {
            console::log_1(&"Piece is being taken!".into());

            modified_active_pieces.remove(&get_key_for_unit(active_piece_being_jumped));
        }

        unit_to_move.coordinate.x = to_x;
        unit_to_move.coordinate.y = to_y;

        modified_active_pieces.insert(get_key_for_unit(&unit_to_move), unit_to_move);
    }

    modified_active_pieces
}