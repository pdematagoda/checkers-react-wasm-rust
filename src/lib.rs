use crate::logic::board_converter::from_internal_board;
use crate::logic::board_converter::to_internal_board;
use wasm_bindgen::prelude::*;

mod logic;

use self::logic::models::{ Board, Unit, Colour, Side, Coordinate, UnitType };
use self::logic::board_generator::generate_board;
use self::logic::board_mover::do_move;

#[wasm_bindgen]
pub fn generateBoard() -> Board {
    generate_board()
}

#[wasm_bindgen]
pub fn doMove(board: Board, unit: Unit, to_x: u8, to_y: u8) -> Board {
    let internal_board = to_internal_board(board);

    from_internal_board(do_move(internal_board, unit, to_x, to_y))
}