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

    let mut x_change = (to_x) - (unit.coordinate.x);
    let mut y_change = (to_y) - (unit.coordinate.y);

    if (unit.colour == Colour::Black) {
        x_change = -x_change;
        y_change = -y_change;
    }

    return x_change == 1 && y_change == 1;
}

fn validate_move(active_pieces: &HashMap<String, Unit>, piece: &Unit, to_x: i8, to_y: i8) -> bool {
    if (!can_do_move(&piece, to_x, to_y)) {
        return false;
    }

    let piece_moving_to_key = get_key_for_x_and_y(to_x, to_y);

    let active_piece_at_key = active_pieces.get(&piece_moving_to_key);

    if let Some(active_piece_at_key) = active_piece_at_key {
        return !(active_piece_at_key.colour == piece.colour);
    }

    true
}

fn get_valid_moves(active_pieces: &HashMap<String, Unit>, piece: &Unit) -> Vec<Coordinate> {
    let mut possible_moves = Vec::new();

    let can_move_forward = piece.colour == Colour::White;
    let y_change = if can_move_forward { 1 } else { -1 };

    if can_move_forward {
        if (piece.coordinate.y < BoardLength) {
            if (piece.coordinate.x < BoardWidth) {
                possible_moves.push(Coordinate {
                    x: piece.coordinate.x + 1,
                    y: piece.coordinate.y + y_change,
                })
            }

            if (piece.coordinate.x > 1) {
                possible_moves.push(Coordinate {
                    x: piece.coordinate.x - 1,
                    y: piece.coordinate.y + y_change,
                })
            }
        }
    } else {
        if (piece.coordinate.y > 1) {
            if (piece.coordinate.x < BoardWidth) {
                possible_moves.push(Coordinate {
                    x: piece.coordinate.x + 1,
                    y: piece.coordinate.y + y_change,
                })
            }

            if (piece.coordinate.x > 1) {
                possible_moves.push(Coordinate {
                    x: piece.coordinate.x - 1,
                    y: piece.coordinate.y + y_change,
                })
            }
        }
    }

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

    let (moving_piece, move_to_take) = find_next_move(&active_pieces, opposing_pieces);

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

        let unit_at_to = modified_active_pieces.remove(&get_key_for_x_and_y(to_x, to_y));

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
}
