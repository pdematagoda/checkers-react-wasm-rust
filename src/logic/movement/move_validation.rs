use crate::logic::models::{Colour, Unit, BOARD_LENGTH, BOARD_WIDTH};
use crate::logic::movement::utilities::{get_active_piece_at_x_and_y, get_piece_being_jumped};
use std::collections::HashMap;

fn can_do_move(unit: &Unit, to_x: i8, to_y: i8) -> bool {
    if to_x < 1 || to_y < 1 {
        return false;
    }

    if to_x > BOARD_WIDTH || to_y > BOARD_LENGTH {
        return false;
    }

    let x_change = to_x - unit.coordinate.x;
    let y_change = to_y - unit.coordinate.y;

    let is_valid_colour_move = match unit.colour {
        Colour::Black => y_change < 0,
        Colour::White => y_change > 0,
    };

    if !is_valid_colour_move {
        return false;
    }

    let x_change = x_change.abs();
    let y_change = y_change.abs();

    return (x_change == 1 && y_change == 1) || (x_change == 2 && y_change == 2);
}

pub fn validate_move(
    active_pieces: &HashMap<String, Unit>,
    piece: &Unit,
    to_x: i8,
    to_y: i8,
) -> bool {
    if !can_do_move(&piece, to_x, to_y) {
        return false;
    }

    let active_piece_at_key = get_active_piece_at_x_and_y(active_pieces, to_x, to_y);

    if let Some(_active_piece_at_key) = active_piece_at_key {
        return false;
    }

    if let Some(active_piece_being_jumped) =
        get_piece_being_jumped(active_pieces, piece, to_x, to_y)
    {
        if active_piece_being_jumped.colour == piece.colour {
            return false;
        }
    } else {
        let is_jump = (piece.coordinate.y - to_y).abs() == 2;

        if is_jump {
            return false;
        }
    }

    true
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
    fn white_pawn_cannot_move_backward() {
        assert_eq!(can_do_move(&get_white_unit_with_coords(4, 4), 3, 3), false);
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
    fn black_pawn_cannot_move_backward() {
        assert_eq!(can_do_move(&get_black_unit_with_coords(6, 2), 5, 3), false);
    }

    #[test]
    fn black_can_jump_forward_right() {
        assert_eq!(can_do_move(&get_black_unit_with_coords(6, 6), 8, 4), true);
    }

    #[test]
    fn black_cannot_move_forward_outside_the_board() {
        assert_eq!(can_do_move(&get_black_unit_with_coords(8, 8), 9, 9), false);
    }
}
