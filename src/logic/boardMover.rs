use wasm_bindgen::prelude::*;

use crate::logic::models::{ Board, Unit };

pub fn do_move(board: Board, unit: Unit, to_x: u8, to_y: u8) -> Board {
    let mut resulting_board = board.clone();

    resulting_board
}