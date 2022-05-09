use crate::logic::models::InternalBoard;
use std::collections::HashMap;
use crate::logic::models::{ Board, Unit, Colour, get_key_for_unit, get_key_for_x_and_y };

use web_sys::console;

pub fn can_do_move(unit: Unit, to_x: u8, to_y: u8) -> bool {
    if (to_x < 1 || to_y < 1) {
        return false;
    }

    let mut x_change = (to_x as i32) - (unit.coordinate.x as i32);
    let mut y_change = (to_y as i32) - (unit.coordinate.y as i32);

    if (unit.colour == Colour::Black) {
        x_change = -x_change;
        y_change = -y_change;
    }

    return x_change == 1 && y_change == 1;
}

fn perform_move_and_get_active_pieces(active_pieces: HashMap<String, Unit>, unit: Unit, to_x: u8, to_y: u8) -> HashMap<String, Unit> {
    let mut modified_active_pieces = active_pieces.clone();

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

pub fn do_move(board: InternalBoard, unit: Unit, to_x: u8, to_y: u8) -> InternalBoard {    
    InternalBoard {
        active_pieces: perform_move_and_get_active_pieces(board.active_pieces, unit, to_x, to_y),
        human_player: board.human_player
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::models::*;

    fn get_white_unit_with_coords(x: u8, y: u8) -> Unit {
        Unit {
            active: true,
            colour: Colour::White,
            unit_type: UnitType::Pawn,
            coordinate: Coordinate {
                x,
                y
            }
        }
    }

    fn get_black_unit_with_coords(x: u8, y: u8) -> Unit {
        Unit {
            active: true,
            colour: Colour::Black,
            unit_type: UnitType::Pawn,
            coordinate: Coordinate {
                x,
                y
            }
        }
    }

    #[test]
    fn white_can_move_forward() {
        assert_eq!(can_do_move(get_white_unit_with_coords(1, 1), 2, 2), true);
    }

    #[test]
    fn white_cannot_move_forward_straight() {
        assert_eq!(can_do_move(get_white_unit_with_coords(1, 1), 1, 2), false);
    }

    #[test]
    fn white_cannot_move_backward_straight() {
        assert_eq!(can_do_move(get_white_unit_with_coords(5, 5), 4, 5), false);
    }

    #[test]
    fn white_cannot_move_backwards_from_a_corner() {
        assert_eq!(can_do_move(get_white_unit_with_coords(1, 1), 0, 1), false);
    }

    #[test]
    fn black_can_move_forward() {
        assert_eq!(can_do_move(get_black_unit_with_coords(6, 2), 5, 1), true);
    }
}