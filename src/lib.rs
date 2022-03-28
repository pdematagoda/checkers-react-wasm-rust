use wasm_bindgen::prelude::*;

mod logic;

use self::logic::models::{ Board, Unit, Colour, Side, Coordinate, UnitType };
use self::logic::boardGenerator::generate_board;
use self::logic::boardMover::do_move;

#[wasm_bindgen]
pub fn generateBoard() -> Board {
    generate_board()
}

#[wasm_bindgen]
pub fn doMove(board: Board, unit: Unit, to_x: u8, to_y: u8) -> Board {
    do_move(board, unit, to_x, to_y)
}