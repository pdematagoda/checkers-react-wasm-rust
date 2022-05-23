use crate::logic::board_converter::from_internal_board;
use crate::logic::board_converter::to_internal_board;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

mod logic;

use self::logic::board_generator::generate_board as generate_board_internal;
use self::logic::board_mover::{
    do_move as do_move_internal, is_valid_board_move as is_valid_board_move_internal,
};
use self::logic::models::{Board, Unit};

#[wasm_bindgen(js_name = initialiseEngine)]
pub fn initialise_engine() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen(js_name = generateBoard)]
pub fn generate_board() -> Board {
    generate_board_internal()
}

#[wasm_bindgen(js_name = copyBoard)]
pub fn copy_board(board: &Board) -> Board {
    board.clone()
}

#[wasm_bindgen(js_name = doMove)]
pub fn do_move(board: Board, unit: Unit, to_x: i8, to_y: i8) -> Board {
    let internal_board = to_internal_board(board);

    from_internal_board(do_move_internal(internal_board, unit, to_x, to_y))
}

#[wasm_bindgen(js_name = isValidBoardMove)]
pub fn is_valid_board_move(board: &Board, unit: &Unit, to_x: i8, to_y: i8) -> bool {
    let internal_board = to_internal_board(board.clone());

    is_valid_board_move_internal(&internal_board, unit, to_x, to_y)
}
