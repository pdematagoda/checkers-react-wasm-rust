use crate::logic::board_converter::from_internal_board;
use crate::logic::board_converter::to_internal_board;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

mod logic;

use self::logic::board_generator::generate_board;
use self::logic::board_mover::do_move;
use self::logic::models::{Board, Colour, Coordinate, Side, Unit, UnitType};

#[wasm_bindgen]
pub fn initialiseEngine() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn generateBoard() -> Board {
    generate_board()
}

#[wasm_bindgen]
pub fn doMove(board: Board, unit: Unit, to_x: i8, to_y: i8) -> Board {
    let internal_board = to_internal_board(board);

    from_internal_board(do_move(internal_board, unit, to_x, to_y))
}
