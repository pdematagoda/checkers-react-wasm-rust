use crate::logic::models::{get_key_for_x_and_y, Unit};
use std::collections::HashMap;
use web_sys::console;

pub fn get_active_piece_at_x_and_y(
    active_pieces: &HashMap<String, Unit>,
    to_x: i8,
    to_y: i8,
) -> Option<&Unit> {
    let piece_moving_to_key = get_key_for_x_and_y(to_x, to_y);

    let active_piece_at_key = active_pieces.get(&piece_moving_to_key);

    active_piece_at_key
}

pub fn get_piece_being_jumped<'a>(
    active_pieces: &'a HashMap<String, Unit>,
    piece: &Unit,
    to_x: i8,
    to_y: i8,
) -> Option<&'a Unit> {
    let is_jump = (piece.coordinate.y - to_y).abs() == 2;

    if is_jump {
        let jumping_over_x = piece.coordinate.x + ((to_x - piece.coordinate.x) / 2);
        let jumping_over_y = piece.coordinate.y + ((to_y - piece.coordinate.y) / 2);

        console::log_1(
            &format!(
                "For piece at {}x and {}y, the jumped unit is at {}x and {}y",
                piece.coordinate.x, piece.coordinate.y, to_x, to_y
            )
            .into(),
        );

        let active_piece_being_jumped =
            get_active_piece_at_x_and_y(active_pieces, jumping_over_x, jumping_over_y);

        return active_piece_being_jumped;
    }

    None
}
