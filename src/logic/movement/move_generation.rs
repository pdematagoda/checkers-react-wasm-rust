use crate::logic::models::{get_key_for_x_and_y, Colour, Coordinate, UnitType};
use crate::logic::movement::move_validation::validate_move;
use crate::Unit;
use std::collections::HashMap;

fn get_y_change_for_colour(colour: Colour) -> i8 {
    let can_move_forward = colour == Colour::White;
    let y_change = if can_move_forward { 1 } else { -1 };

    y_change
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

fn get_jumping_coordinate_if_required(
    active_pieces: &HashMap<String, Unit>,
    piece: &Unit,
    possible_move: Coordinate,
) -> Coordinate {
    if let Some(_active_piece_at_move) =
        get_active_piece_at_x_and_y(active_pieces, possible_move.x, possible_move.y)
    {
        let y_change = (possible_move.y - piece.coordinate.y) * 2;
        let x_change = (possible_move.x - piece.coordinate.x) * 2;

        return Coordinate {
            x: piece.coordinate.x + x_change,
            y: piece.coordinate.y + y_change,
        };
    }

    possible_move
}

pub fn get_potential_moves(active_pieces: &HashMap<String, Unit>, piece: &Unit) -> Vec<Coordinate> {
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

    if piece.unit_type == UnitType::King {
        let y_change = y_change * -1;

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
    }

    possible_moves
}

pub fn get_valid_moves(active_pieces: &HashMap<String, Unit>, piece: &Unit) -> Vec<Coordinate> {
    let possible_moves = get_potential_moves(active_pieces, piece);

    let mut valid_moves = Vec::new();

    for possible_move in possible_moves.iter() {
        if validate_move(active_pieces, piece, possible_move.x, possible_move.y) {
            valid_moves.push(*possible_move);
        }
    }

    valid_moves
}

mod tests {
    use super::*;
    use crate::logic::models::*;

    fn get_white_pawn_unit_with_coords(x: i8, y: i8) -> Unit {
        Unit {
            active: true,
            colour: Colour::White,
            unit_type: UnitType::Pawn,
            coordinate: Coordinate { x, y },
        }
    }

    fn get_white_king_unit_with_coords(x: i8, y: i8) -> Unit {
        Unit {
            active: true,
            colour: Colour::White,
            unit_type: UnitType::King,
            coordinate: Coordinate { x, y },
        }
    }

    fn get_black_king_unit_with_coords(x: i8, y: i8) -> Unit {
        Unit {
            active: true,
            colour: Colour::Black,
            unit_type: UnitType::King,
            coordinate: Coordinate { x, y },
        }
    }

    fn get_black_pawn_unit_with_coords(x: i8, y: i8) -> Unit {
        Unit {
            active: true,
            colour: Colour::Black,
            unit_type: UnitType::Pawn,
            coordinate: Coordinate { x, y },
        }
    }

    #[test]
    fn get_valid_moves_for_a_middle_white_pawn_piece_with_no_other_pieces_nearby() {
        let valid_moves = get_valid_moves(&HashMap::new(), &get_white_pawn_unit_with_coords(4, 4));

        assert_eq!(
            valid_moves,
            vec![Coordinate { x: 5, y: 5 }, Coordinate { x: 3, y: 5 }]
        );
    }

    #[test]
    fn get_valid_moves_for_a_middle_white_king_piece_with_no_other_pieces_nearby() {
        let valid_moves = get_valid_moves(&HashMap::new(), &get_white_king_unit_with_coords(4, 4));

        assert_eq!(
            valid_moves,
            vec![
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 3, y: 5 },
                Coordinate { x: 5, y: 3 },
                Coordinate { x: 3, y: 3 }
            ]
        );
    }

    #[test]
    fn get_valid_moves_for_a_middle_black_pawn_piece_with_no_other_pieces_nearby() {
        let valid_moves = get_valid_moves(&HashMap::new(), &get_black_pawn_unit_with_coords(4, 4));

        assert_eq!(
            valid_moves,
            vec![Coordinate { x: 5, y: 3 }, Coordinate { x: 3, y: 3 }]
        );
    }

    #[test]
    fn get_valid_moves_for_a_middle_black_king_piece_with_no_other_pieces_nearby() {
        let valid_moves = get_valid_moves(&HashMap::new(), &get_black_king_unit_with_coords(4, 4));

        assert_eq!(
            valid_moves,
            vec![
                Coordinate { x: 5, y: 3 },
                Coordinate { x: 3, y: 3 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 3, y: 5 }
            ]
        );
    }
}
