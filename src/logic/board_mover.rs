use crate::logic::models::BoardLength;
use crate::logic::models::BoardWidth;
use crate::logic::models::{
    get_key_for_unit, get_key_for_x_and_y, Board, Colour, Coordinate, InternalBoard, Unit,
};
use std::collections::HashMap;

use js_sys::Math::random;
use web_sys::console;

pub fn can_do_move(unit: &Unit, to_x: i8, to_y: i8) -> bool {
    if (to_x < 1 || to_y < 1) {
        return false;
    }

    if (to_x > BoardWidth || to_y > BoardLength) {
        return false;
    }

    let x_change = (to_x - unit.coordinate.x).abs();
    let y_change = (to_y - unit.coordinate.y).abs();

    return (x_change == 1 && y_change == 1) || (x_change == 2 && y_change == 2);
}

fn get_active_piece_at_x_and_y(
    active_pieces: &HashMap<String, Unit>,
    to_x: i8,
    to_y: i8,
) -> Option<&Unit> {
    let piece_moving_to_key = get_key_for_x_and_y(to_x, to_y);

    let active_piece_at_key = active_pieces.get(&piece_moving_to_key);

    active_piece_at_key
}

fn get_piece_being_jumped<'a>(
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

fn validate_move(active_pieces: &HashMap<String, Unit>, piece: &Unit, to_x: i8, to_y: i8) -> bool {
    if (!can_do_move(&piece, to_x, to_y)) {
        return false;
    }

    let active_piece_being_jumped = get_piece_being_jumped(active_pieces, piece, to_x, to_y);

    if let Some(active_piece_being_jumped) = active_piece_being_jumped {
        if !(active_piece_being_jumped.colour == piece.colour) {
            return true;
        }
        return false;
    }

    let active_piece_at_key = get_active_piece_at_x_and_y(active_pieces, to_x, to_y);

    if let Some(_active_piece_at_key) = active_piece_at_key {
        return false;
    }

    true
}

fn get_y_change_for_colour(colour: Colour) -> i8 {
    let can_move_forward = colour == Colour::White;
    let y_change = if can_move_forward { 1 } else { -1 };

    y_change
}

fn get_jumping_coordinate_if_required(
    active_pieces: &HashMap<String, Unit>,
    piece: &Unit,
    possible_move: Coordinate,
) -> Coordinate {
    if let Some(_active_piece_at_move) =
        get_active_piece_at_x_and_y(active_pieces, possible_move.x, possible_move.y)
    {
        let y_change = get_y_change_for_colour(piece.colour) * 2;
        let x_change = (piece.coordinate.x - possible_move.x) * 2;

        console::log_1(&format!("Trying a jump!").into());

        return Coordinate {
            x: piece.coordinate.x + x_change,
            y: piece.coordinate.y + y_change,
        };
    }

    possible_move
}

fn get_potential_moves(active_pieces: &HashMap<String, Unit>, piece: &Unit) -> Vec<Coordinate> {
    let mut possible_moves = Vec::new();

    let y_change = get_y_change_for_colour(piece.colour);

    possible_moves.push(get_jumping_coordinate_if_required(
        active_pieces,
        piece,
        Coordinate {
            x: piece.coordinate.x + 1,
            y: piece.coordinate.y + y_change,
        },
    ));

    possible_moves.push(get_jumping_coordinate_if_required(
        active_pieces,
        piece,
        Coordinate {
            x: piece.coordinate.x - 1,
            y: piece.coordinate.y + y_change,
        },
    ));

    possible_moves
}

fn get_valid_moves(active_pieces: &HashMap<String, Unit>, piece: &Unit) -> Vec<Coordinate> {
    let possible_moves = get_potential_moves(active_pieces, piece);

    let mut valid_moves = Vec::new();

    for possible_move in possible_moves.iter() {
        if (validate_move(active_pieces, piece, possible_move.x, possible_move.y)) {
            valid_moves.push(*possible_move);
        }
    }

    valid_moves
}

fn get_active_pieces_for_side(active_pieces: &HashMap<String, Unit>, side: Colour) -> Vec<Unit> {
    let mut pieces = Vec::new();

    for (_, piece) in active_pieces {
        if (piece.colour == side) {
            pieces.push(*piece);
        }
    }

    pieces
}

fn get_pieces_with_possible_moves(
    active_pieces: &HashMap<String, Unit>,
    pieces_to_move: Vec<Unit>,
) -> Vec<Unit> {
    let mut pieces_that_can_move = pieces_to_move.clone();

    console::log_1(&format!("Starting with {} pieces", pieces_to_move.len()).into());

    pieces_that_can_move
        .retain(|&piece_to_check| get_valid_moves(&active_pieces, &piece_to_check).len() > 0);

    console::log_1(
        &format!(
            "Have {} pieces that can actually move",
            pieces_that_can_move.len()
        )
        .into(),
    );

    pieces_that_can_move
}

fn find_next_move(
    active_pieces: &HashMap<String, Unit>,
    pieces_to_move: Vec<Unit>,
) -> (Unit, Coordinate) {
    let piece_index = (random() * 100.0) as usize % pieces_to_move.len();
    let piece_to_move = pieces_to_move[piece_index];

    let possible_moves = get_valid_moves(&active_pieces, &piece_to_move);

    if (possible_moves.len() == 0) {
        console::log_1(&"Need to try again.....".into());

        let mut new_pieces_to_move = pieces_to_move.clone();

        new_pieces_to_move.remove(piece_index);

        return find_next_move(active_pieces, new_pieces_to_move);
    }

    let selected_move = possible_moves[(random() * 100.0) as usize % possible_moves.len()];

    return (piece_to_move, selected_move);
}

fn do_opposing_move(
    active_pieces: HashMap<String, Unit>,
    moved_piece: Unit,
) -> HashMap<String, Unit> {
    let opposing_colour = match moved_piece.colour {
        Colour::Black => Colour::White,
        Colour::White => Colour::Black,
    };

    console::log_1(&"Moving opposing unit".into());

    let opposing_pieces = get_active_pieces_for_side(&active_pieces, opposing_colour);

    let (moving_piece, move_to_take) = find_next_move(
        &active_pieces,
        get_pieces_with_possible_moves(&active_pieces, opposing_pieces),
    );

    return perform_move_and_get_active_pieces(
        active_pieces,
        moving_piece,
        move_to_take.x,
        move_to_take.y,
    );
}

fn perform_move_and_get_active_pieces(
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

pub fn do_move(board: InternalBoard, unit: Unit, to_x: i8, to_y: i8) -> InternalBoard {
    let active_pieces = perform_move_and_get_active_pieces(board.active_pieces, unit, to_x, to_y);

    InternalBoard {
        active_pieces: do_opposing_move(active_pieces, unit),
        human_player: board.human_player,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::models::*;

    #[test]
    fn get_potential_moves_for_a_middle_white_piece_with_no_other_pieces_nearby() {}

    #[test]
    fn get_potential_moves_for_a_middle_black_piece_with_no_other_pieces_nearby() {}

    fn get_white_unit_with_coords(x: i8, y: i8) -> Unit {
        Unit {
            active: true,
            colour: Colour::White,
            unit_type: UnitType::Pawn,
            coordinate: Coordinate { x, y },
        }
    }

    fn get_black_unit_with_coords(x: i8, y: i8) -> Unit {
        Unit {
            active: true,
            colour: Colour::Black,
            unit_type: UnitType::Pawn,
            coordinate: Coordinate { x, y },
        }
    }

    #[test]
    fn white_cannot_move_backwards_out_of_the_board() {
        assert_eq!(can_do_move(&get_white_unit_with_coords(1, 1), 0, 0), false);
    }

    #[test]
    fn white_can_move_forward() {
        assert_eq!(can_do_move(&get_white_unit_with_coords(1, 1), 2, 2), true);
    }

    #[test]
    fn white_cannot_move_forward_straight() {
        assert_eq!(can_do_move(&get_white_unit_with_coords(1, 1), 1, 2), false);
    }

    #[test]
    fn white_cannot_move_backward_straight() {
        assert_eq!(can_do_move(&get_white_unit_with_coords(5, 5), 4, 5), false);
    }

    #[test]
    fn white_cannot_move_backwards_from_a_corner() {
        assert_eq!(can_do_move(&get_white_unit_with_coords(1, 1), 0, 1), false);
    }

    #[test]
    fn black_can_move_forward() {
        assert_eq!(can_do_move(&get_black_unit_with_coords(6, 2), 5, 1), true);
    }

    #[test]
    fn black_can_jump_forward_right() {
        assert_eq!(can_do_move(&get_black_unit_with_coords(6, 6), 4, 8), true);
    }

    #[test]
    fn black_cannot_move_forward_outside_the_board() {
        assert_eq!(can_do_move(&get_black_unit_with_coords(8, 8), 9, 9), false);
    }
}
