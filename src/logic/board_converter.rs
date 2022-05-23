use crate::logic::models::{
    get_key_for_unit, Board, Colour, Coordinate, InternalBoard, Side, Unit, UnitType,
};
use std::collections::HashMap;

use web_sys::console;

fn add_unit_to_active_pieces(unit: Unit, pieces: &mut HashMap<String, Unit>) {
    if unit.active {
        pieces.insert(get_key_for_unit(&unit), unit);
    }
}

fn add_side_to_active_pieces(side: Side, pieces: &mut HashMap<String, Unit>) {
    add_unit_to_active_pieces(side.one, pieces);
    add_unit_to_active_pieces(side.two, pieces);
    add_unit_to_active_pieces(side.three, pieces);
    add_unit_to_active_pieces(side.four, pieces);
    add_unit_to_active_pieces(side.five, pieces);
    add_unit_to_active_pieces(side.six, pieces);
    add_unit_to_active_pieces(side.seven, pieces);
    add_unit_to_active_pieces(side.eight, pieces);
    add_unit_to_active_pieces(side.nine, pieces);
    add_unit_to_active_pieces(side.ten, pieces);
    add_unit_to_active_pieces(side.eleven, pieces);
    add_unit_to_active_pieces(side.twelve, pieces);
}

pub fn to_internal_board(board: Board) -> InternalBoard {
    let mut map_of_active_pieces = HashMap::new();

    add_side_to_active_pieces(board.black_pieces, &mut map_of_active_pieces);
    add_side_to_active_pieces(board.white_pieces, &mut map_of_active_pieces);

    InternalBoard {
        human_player: board.human_player,
        active_pieces: map_of_active_pieces,
    }
}

fn get_unit(side_pieces: &Vec<Unit>, index: usize, side: Colour) -> Unit {
    let empty_piece: Unit = Unit {
        active: false,
        colour: side,
        coordinate: Coordinate { x: 1, y: 1 },
        unit_type: UnitType::Pawn,
    };

    if index >= side_pieces.len() {
        console::log_1(&"Returning empty piece".into());

        return empty_piece;
    }

    console::log_1(&"Returning piece".into());

    side_pieces[index]
}

fn get_side_for_active_pieces(active_pieces: &HashMap<String, Unit>, side: Colour) -> Side {
    let mut side_pieces: Vec<Unit> = Vec::new();

    for (_key, unit) in active_pieces {
        if unit.colour == side && unit.active {
            console::log_1(&"Pushing active unit".into());

            side_pieces.push(*unit);
        }
    }

    Side {
        one: get_unit(&side_pieces, 0, side),
        two: get_unit(&side_pieces, 1, side),
        three: get_unit(&side_pieces, 2, side),
        four: get_unit(&side_pieces, 3, side),
        five: get_unit(&side_pieces, 4, side),
        six: get_unit(&side_pieces, 5, side),
        seven: get_unit(&side_pieces, 6, side),
        eight: get_unit(&side_pieces, 7, side),
        nine: get_unit(&side_pieces, 8, side),
        ten: get_unit(&side_pieces, 9, side),
        eleven: get_unit(&side_pieces, 10, side),
        twelve: get_unit(&side_pieces, 11, side),
    }
}

pub fn from_internal_board(internal_board: InternalBoard) -> Board {
    Board {
        human_player: internal_board.human_player,
        black_pieces: get_side_for_active_pieces(&internal_board.active_pieces, Colour::Black),
        white_pieces: get_side_for_active_pieces(&internal_board.active_pieces, Colour::White),
    }
}
